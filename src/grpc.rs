use dominoes::game_service_server::*;
use dominoes::*;
use std::{error::Error, io::ErrorKind, net::ToSocketAddrs, pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

pub mod dominoes {
    tonic::include_proto!("dominoes"); // The string specified here must match the proto package name
}

type MessageStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[derive(Debug, Default)]
pub struct GrpcServer {}

#[tonic::async_trait]
impl GameService for GrpcServer {
    type joinGameStream = MessageStream;

    async fn join_game(
        &self,
        request: tonic::Request<tonic::Streaming<Message>>,
    ) -> Result<tonic::Response<Self::joinGameStream>, tonic::Status> {
        unimplemented!()
    }
}

pub async fn start_grpc() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = GrpcServer::default();

    Server::builder()
        .add_service(GameServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
