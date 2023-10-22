use crate::game::*;
use async_trait::async_trait;
use tokio::sync::mpsc::{channel, Sender};

#[derive(Debug)]
pub enum Message<'a> {
    Init((&'a Vec<Domino>, usize)),
    YourTurn,
    Update(Update),
}

#[async_trait]
pub trait RemotePlayer: Send + Sync {
    async fn send_message<'a>(&mut self, message: Message<'a>);
    async fn read_move(&mut self) -> Move;
    fn number(&self) -> usize;
    fn set_number(&mut self, number: usize);
}

pub fn start_game() -> Sender<Box<dyn RemotePlayer>> {
    let (tx, mut rx) = channel::<Box<dyn RemotePlayer>>(4);

    tokio::spawn(async move {
        let mut players = Vec::<Box<dyn RemotePlayer>>::new();

        loop {
            let player_opt = rx.recv().await;
            if let Some(mut player) = player_opt {
                player.set_number(players.len());
                players.push(player);
            }

            if players.len() == 4 {
                break;
            }
        }

        rx.close();

        start_game_loop(players).await;
    });

    tx
}

async fn start_game_loop(mut players: Vec<Box<dyn RemotePlayer>>) {
    let mut game = Game::new(players.len() as i32);

    for player in &mut players {
        let pieces = &game.players[player.number()];
        player
            .send_message(Message::Init((pieces, player.number())))
            .await;
    }

    loop {
        let turn = game.next as usize;
        let player = &mut players[turn];
        let mut move_: Move;
        let update: Update;

        player.send_message(Message::YourTurn).await;

        loop {
            move_ = player.read_move().await;

            let outcome = game.play(&move_);
            match outcome {
                Ok(up) => {
                    update = up;
                    break;
                }
                _ => {}
            }
        }

        println!("{:?}", game.board);

        for i in 0..players.len() {
            if i == turn {
                continue;
            }

            players[i].send_message(Message::Update(update)).await;
        }
    }
}
