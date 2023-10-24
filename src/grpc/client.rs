use tokio::sync::{mpsc, mpsc::Sender};
use tokio_stream::{Stream, wrappers::ReceiverStream, StreamExt};
use anyhow::Result;
use crate::game::Move;use crate::grpc::converters;
use crate::grpc::server::dominoes;
use crate::grpc::server::dominoes::game_service_client::*;

const SKIP_MSG: dominoes::Message = dominoes::Message {
    content: Some(dominoes::message::Content::Skip(dominoes::Skip {})),
};

fn read_move(num_player: i32) -> dominoes::Message {
    use std::io::{stdin, stdout, Write};    

    loop {
        let mut s = String::new();

        print!("Please enter some text: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
    
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
    
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        let move_opt = Move::parse_move(&s, num_player as usize);

        if let Some(move_) = move_opt {
            return converters::move_to_proto(&move_);
        }
    }
}

async fn requests_stream() -> (Sender<dominoes::Message>, impl Stream<Item = dominoes::Message>) {
    let (out_sender, out_rec) = mpsc::channel(128);
    
    let stream = ReceiverStream::new(out_rec);
    let _ = out_sender.send(SKIP_MSG).await;
    (out_sender, stream)
}

async fn start_game_loop(sender: Sender<dominoes::Message>, mut stream: tonic::Streaming<dominoes::Message>, 
    message: dominoes::Init) -> Result<()> {
    let player_number = message.number;
    let hand = &message.hand;

    println!("Sua mão: {:?}", hand);

    loop {
        let msg_opt = stream.next().await;

        if let Some(Ok(message)) = msg_opt {
            if let Some(dominoes::message::Content::YouTurn(_)) = message.content {
                let move_ = read_move(player_number);
                let _ = sender.send(move_).await;
            }
            
            if let Some(dominoes::message::Content::Update(update)) = message.content { 
                println!("Update: {:?}", update);
            }
        }        
    }
}

pub async fn start_client() -> Result<()> {
    let mut client = GameServiceClient::connect("http://[::1]:50051").await?;
    let (sender, stream) = requests_stream().await;
    let response = client.join_game(stream).await?;
    let mut resp_stream = response.into_inner();
    let init_opt = resp_stream.next().await;

    if let Some(Ok(message)) = init_opt {
        if let Some(dominoes::message::Content::Init(init_message)) = message.content {
            return start_game_loop(sender, resp_stream, init_message).await
        }
    }

    unimplemented!("Alguma coisa errada não está certa")  
}