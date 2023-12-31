use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub mod hello_world {
    #![allow(non_snake_case)]
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct GreeterService {}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        tracing::info!("Got a request: {:?}", request);

        Ok(Response::new(hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let addr = "127.0.0.1:50051".parse()?;
    println!("Server listening on {}", addr);

    Server::builder()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        // .layer(&cors)
        .accept_http1(true)
        .add_service(tonic_web::enable(GreeterServer::new(GreeterService::default())))
        .serve(addr)
        .await?;

    Ok(())
}
