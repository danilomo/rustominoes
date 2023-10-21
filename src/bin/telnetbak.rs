use anyhow::Result;
use rustominoes::game::*;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use wait_for_me::CountDownLatch;

async fn run_server(socket: TcpStream, number: usize, mut receiver: Receiver<()>) -> Result<()> {
    let mut conn = BufReader::new(socket);
    let mut game = Game::new(4);

    receiver.recv().await;
    let x = format!("{}", number);
    conn.write(x.as_bytes()).await?;

    loop {
        let mut message = String::new();
        conn.read_line(&mut message).await?;

        if let Some(mv) = Move::parse(&message) {
            let result = game.play(&mv);

            if let Ok(_) = result {
                println!("Ok!!!!");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:1234").await?;
    let latch = CountDownLatch::new(4);
    let mut senders = Vec::<Sender<()>>::new();

    loop {
        let (socket, _addr, number) = async {
            let result = listener.accept().await;
            let l = latch.clone();
            let i = l.count().await;
            l.count_down().await;

            match result {
                Ok((stream, addr)) => Ok((stream, addr, i)),
                Err(x) => Err(x),
            }
        }
        .await?;

        let (sender, receiver) = mpsc::channel::<()>(1);
        senders.push(sender);

        tokio::spawn(run_server(socket, number, receiver));

        if latch.count().await == 0 {
            break;
        }
    }

    tokio::spawn(async move {
        latch.wait().await;

        for sender in &senders {
            let _ = sender.send(()).await;
        }
    });

    signal::ctrl_c().await?;

    Ok(())
}
