use crate::game::*;
use async_trait::async_trait;

#[derive(Debug)]
pub enum Message<'a> {
    Init((&'a Vec<Domino>, usize)),
    YourTurn,
    Update(Move),
}

#[async_trait]
pub trait RemotePlayer {
    async fn send_message<'a>(&mut self, message: Message<'a>);
    async fn read_move(&mut self) -> Move;
    fn number(&self) -> usize;
}

pub async fn start_game(mut players: Vec<Box<dyn RemotePlayer>>) {
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

        player.send_message(Message::YourTurn).await;
        loop {
            move_ = player.read_move().await;
            let outcome = game.play(&move_);
            match outcome {
                Ok(()) => {
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
            players[i].send_message(Message::Update(move_)).await;
        }
    }
}
