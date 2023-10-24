use crate::concurrent;
use crate::concurrent::{start_game, RemotePlayer};
use crate::game;
use crate::grpc::converters::*;
use async_trait::async_trait;
use dominoes::game_service_server::*;
use dominoes::*;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Response, Status};

pub mod dominoes {
    tonic::include_proto!("dominoes");
}

type MessageStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[derive(Debug)]
pub struct GrpcServer {
    game_handler: mpsc::Sender<Box<dyn RemotePlayer>>,
}

struct GrpcPlayer {
    receiver: Receiver<Result<Message, Status>>,
    sender: Sender<Result<Message, Status>>,
    number: usize,
}

#[async_trait]
impl RemotePlayer for GrpcPlayer {
    async fn send_message<'a>(&mut self, message: concurrent::Message<'a>) {
        let msg: dominoes::Message = to_proto(&message);
        let _ = self.sender.send(Ok(msg)).await;
    }

    async fn read_move(&mut self) -> game::Move {
        loop {
            let result = self.receiver.recv().await;

            if let Some(Ok(msg_proto)) = result {
                if let Some(move_) = to_move(&msg_proto) {
                    return move_;
                }
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

#[tonic::async_trait]
impl GameService for GrpcServer {
    type JoinGameStream = MessageStream;

    async fn join_game(
        &self,
        request: tonic::Request<tonic::Streaming<Message>>,
    ) -> Result<tonic::Response<Self::JoinGameStream>, tonic::Status> {
        let mut in_stream = request.into_inner();
        let (in_sender, in_rec) = mpsc::channel(128);
        let (out_sender, out_rec) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                let _ = in_sender.send(result).await;
            }
        });

        let remote_player = Box::new(GrpcPlayer {
            receiver: in_rec,
            sender: out_sender,
            number: 0,
        });

        let _ = self.game_handler.send(remote_player).await;

        let output_stream = ReceiverStream::new(out_rec);
        Ok(Response::new(
            Box::pin(output_stream) as Self::JoinGameStream
        ))
    }
}

pub async fn start_grpc() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let game_handler = start_game();

    let greeter = GrpcServer { game_handler };

    Server::builder()
        .add_service(GameServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
