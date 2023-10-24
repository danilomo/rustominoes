use rustominoes::grpc::client::*;
use tokio;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    start_client().await
}
