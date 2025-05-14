use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut count = 0;

    let max_concurrent = 30_000;
    let semaphore = Arc::new(Semaphore::new(max_concurrent));

    loop {
        let permit = semaphore.clone().acquire_owned().await?;
        count += 1;

        tokio::spawn(async move {
            let client_clone =GreeterClient::connect("http://10.0.0.4:5000").await.unwrap();

            let request = tonic::Request::new(HelloRequest {
                name: format!("User{}", count),
                age: 25 + (count % 10),
                subscribed: true,
                rating: 3.5 + (count as f32 % 5.0),
                tags: vec!["rust".into(), "parallel".into()],
                avatar: vec![0x01, 0x02, 0x03],
            });

            match client_clone.clone().say_hello(request).await {
                Ok(response) => {
                    let reply = response.into_inner();
                    // println!(
                    //     "[Thread {}] Reply: {}, Score: {}, Metadata: {:?}",
                    //     count, reply.message, reply.score, reply.metadata
                    // );
                }
                Err(e) => {
                    eprintln!("[Thread {}] Request failed: {}", count, e);
                }
            }

            drop(permit);
        });

        //sleep(Duration::from_micros(1)).await;
    }
}
