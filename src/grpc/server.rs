#![allow(unused_imports)]
#![allow(dead_code)]

use crate::concurrent;
use crate::concurrent::{start_game, RemotePlayer};
use crate::game;
use async_trait::async_trait;
use dominoes::game_service_server::*;
use dominoes::*;
use std::{error::Error, io::ErrorKind, net::ToSocketAddrs, pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time;
use tokio::time::sleep;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};
use std::sync::Arc;

pub mod dominoes {
    tonic::include_proto!("dominoes"); // The string specified here must match the proto package name
}

type MessageStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[derive(Debug)]
pub struct GrpcServer {
    game_handler: mpsc::Sender<Box<dyn RemotePlayer>>,
}

struct GrpcPlayer {
    out_stream: Sender<Result<Message, Status>>,
    number: usize,
}

#[async_trait]
impl RemotePlayer for GrpcPlayer {

    async fn send_message<'a>(&mut self, message: concurrent::Message<'a>) {
        unimplemented!()
    }

    async fn read_move(&mut self) -> game::Move {
        unimplemented!()
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

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
            }
        });

        unimplemented!()     
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
