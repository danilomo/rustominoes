use anyhow::Result;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::signal;
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use wait_for_me::CountDownLatch;
use rustominoes::concurrent::*;
use rustominoes::game::Move;
use async_trait::async_trait;
use std::sync::Arc;

struct TelnetPlayer {
    buf_reader: BufReader<TcpStream>,
    number: usize
}

#[async_trait]
impl RemotePlayer for TelnetPlayer {

    async fn send_message<'a>(&mut self, message: Message<'a>) {
        let message = format!("{:?}\n", message);
        let _ = self.buf_reader.write(message.as_bytes()).await;
    }

    async fn read_move(&mut self) -> Move {
        loop {
            let mut message = String::new();  
            if let Err(_) = self.buf_reader.read_line(&mut message).await {
                continue;
            }

            if let Some(mv) = Move::parse_move(&message, self.number) {
                return mv;
            }
        }
    }

    fn number(&self) -> usize {
        self.number
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:1234").await?;
    let latch = CountDownLatch::new(4);
    let mut connections = Vec::<(BufReader<TcpStream>, usize)>::new();

    loop {
        let (socket, _addr, number) = async {
            let result = listener.accept().await;
            let l = latch.clone();
            let i = l.count().await;
            l.count_down().await;
            
            match result {
                Ok((stream, addr)) => Ok((stream, addr, i)),
                Err(x) => Err(x)
            }
        }.await?;

        connections.push((BufReader::new(socket), number));

        if latch.count().await == 0 {
            break;
        }
    }

    latch.wait().await;
    let mut players: Vec<Box<dyn RemotePlayer>> = Vec::new();

    for elem in connections {
        let player = Box::new(TelnetPlayer{number: elem.1 - 1, buf_reader: elem.0});
        players.push(player);
    }

    start_game(players).await;    

    Ok(())
}
