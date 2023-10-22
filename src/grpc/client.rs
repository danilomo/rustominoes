// client
/*
use dominoes::game_service_client::GameServiceClient;

pub async fn join_game() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GameServiceClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    let msg = Message::default();

    let (tx, rx) = mpsc::channel(128);
    let output_stream = ReceiverStream::new(rx);

    let _ = tx.send(msg).await?;

    let x = client.join_game(output_stream).await.unwrap().into_inner();

    Ok(())
}




        let xxx = in_stream.next().await;

        println!(">>> {:?}", xxx);

        let (tx, rx) = mpsc::channel(128);
        let output_stream = ReceiverStream::new(rx);

        tokio::spawn(async move {
            let m = Ok(Message::default());

            sleep(time::Duration::from_millis(100)).await;
            let _ = tx.send(m).await;
        });

        Ok(Response::new(
            Box::pin(output_stream) as Self::JoinGameStream
        ))

*/
