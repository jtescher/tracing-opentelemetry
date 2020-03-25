use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use opentelemetry::api::{self, HttpTextFormat, KeyValue, Provider};
use opentelemetry::sdk::Sampler;
use opentelemetry::{global, sdk};
use tracing_opentelemetry::{OpenTelemetryLayer, OpenTelemetrySpanExt};
use tracing_subscriber::{Layer, Registry};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

fn tracing_init() -> Result<(), Box<dyn std::error::Error>> {
    let builder = opentelemetry_jaeger::Exporter::builder()
        .with_agent_endpoint("127.0.0.1:6831".parse().unwrap());

    let exporter = builder
        .with_process(opentelemetry_jaeger::Process {
            service_name: "server".to_string(),
            tags: vec![KeyValue::new("version", "0.1.0")],
        })
        .init()?;

    let provider = sdk::Provider::builder()
        .with_simple_exporter(exporter)
        .with_config(sdk::Config {
            default_sampler: Box::new(Sampler::Always),
            ..Default::default()
        })
        .build();

    let tracer = provider.get_tracer("my-tracer");
    let opentelemetry = OpenTelemetryLayer::with_tracer(tracer);

    let subscriber = Registry::default()
         // add the OpenTelemetry subscriber layer
        .with(opentelemetry)
         // add a logging layer
        .with(tracing_subscriber::fmt::Layer::default())
         // add RUST_LOG-based filtering
        .with(tracing_subscriber::EnvFilter::from_default_env());
    tracing::subscriber::set_global_default(subscriber)?;
    global::set_provider(provider);

    Ok(())
}

struct HttpHeaderMapCarrier<'a>(&'a http::HeaderMap);
impl<'a> api::Carrier for HttpHeaderMapCarrier<'a> {
    fn get(&self, key: &'static str) -> Option<&str> {
        self.0
            .get(key.to_lowercase().as_str())
            .and_then(|value| value.to_str().ok())
    }

    fn set(&mut self, _key: &'static str, _value: String) {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_init()?;
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    let propagator = api::B3Propagator::new(true);

    Server::builder()
        .trace_fn(move |header| {
            let parent = propagator.extract(&HttpHeaderMapCarrier(header));
            let tracing_span = tracing::info_span!("Received request");
            tracing_span.set_opentelemetry_parent(parent);
            tracing_span
        })
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
