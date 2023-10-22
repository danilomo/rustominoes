use anyhow::Result;
use async_trait::async_trait;
use rustominoes::concurrent::*;
use rustominoes::game::Move;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

struct TelnetPlayer {
    buf_reader: BufReader<TcpStream>,
    number: usize,
}

impl TelnetPlayer {
    fn new(buf_reader: BufReader<TcpStream>) -> Box<dyn RemotePlayer> {
        Box::new(TelnetPlayer {
            buf_reader,
            number: 0,
        })
    }
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

    fn set_number(&mut self, number: usize) {
        self.number = number;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:1234").await?;
    let game_handler = start_game();

    loop {
        let (socket, _addr) = listener.accept().await?;

        if !game_handler.is_closed() {
            let player = TelnetPlayer::new(BufReader::new(socket));
            let _ = game_handler.send(player).await?;
        }
    }
}
