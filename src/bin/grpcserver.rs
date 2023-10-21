use rustominoes::grpc::start_grpc;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_grpc().await
}
