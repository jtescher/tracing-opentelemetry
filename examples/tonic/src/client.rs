use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use opentelemetry::api::{B3Propagator, HttpTextFormat, KeyValue, Provider};
use opentelemetry::sdk::Sampler;
use opentelemetry::{api, sdk};
use tracing_opentelemetry::{OpenTelemetryLayer, OpenTelemetrySpanExt};
use tracing_subscriber::{Layer, Registry};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

fn tracing_init() -> Result<(), Box<dyn std::error::Error>> {
    let builder = opentelemetry_jaeger::Exporter::builder()
        .with_agent_endpoint("127.0.0.1:6831".parse().unwrap());

    let exporter = builder
        .with_process(opentelemetry_jaeger::Process {
            service_name: "client".to_string(),
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

    let subscriber = opentelemetry.with_subscriber(Registry::default());
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

struct TonicMetadataMapCarrier<'a>(&'a mut tonic::metadata::MetadataMap);
impl<'a> api::Carrier for TonicMetadataMapCarrier<'a> {
    fn get(&self, key: &'static str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    fn set(&mut self, key: &'static str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.to_lowercase().as_bytes()) {
            self.0.insert(
                key,
                tonic::metadata::MetadataValue::from_str(&value).unwrap(),
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_init()?;
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let propagator = B3Propagator::new(true);
    let request_span = tracing::info_span!("client-request");

    let mut request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    propagator.inject(
        request_span.opentelemetry_context(),
        &mut TonicMetadataMapCarrier(request.metadata_mut()),
    );

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
