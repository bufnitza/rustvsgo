use tonic::{transport::Server, Request, Response, Status};
use helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld::{HelloRequest, HelloReply};
use std::{collections::HashMap, sync::Arc};
use rand::random;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::time::{sleep, Duration};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {
    counter: Arc<AtomicUsize>,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        self.counter.fetch_add(1, Ordering::Relaxed);

        let req = request.into_inner();
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "rust-grpc".to_string());

        let reply = HelloReply {
            message: format!("Hello {}, age {}", req.name, req.age),
            user_id: random(),
            score: req.rating as f64 + 10.0,
            active: req.subscribed,
            lucky_numbers: vec![7, 14, 28],
            metadata,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5000".parse()?;
    let counter = Arc::new(AtomicUsize::new(0));

    let greeter = MyGreeter {
        counter: counter.clone(),
    };

    // spawn background task
    tokio::spawn({
        let counter = counter.clone();
        async move {
            loop {
                sleep(Duration::from_secs(10)).await;
                let total = counter.swap(0, Ordering::Relaxed);
                println!("[Rust Server] Total requests in the past 10 seconds: {}", total);
            }
        }
    });

    println!("Rust server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
