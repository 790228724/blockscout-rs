use crate::config::TracingConfiguration;
use opentelemetry::{sdk::trace::Tracer, trace::TraceError};
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, prelude::*};

pub fn init_logs(config: TracingConfiguration) {
    let stdout = tracing_subscriber::fmt::layer().with_filter(
        tracing_subscriber::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
    );
    let registry = tracing_subscriber::registry()
        // output logs (tracing) to stdout with log level taken from env (default is INFO)
        .with(stdout);
    if config.enable_jaeger {
        let tracer = init_jaeger_tracer(&config.jaeger_agent).expect("failed to init tracer");
        registry
            // output traces to jaeger with default log level (default is DEBUG)
            .with(
                tracing_opentelemetry::layer()
                    .with_tracer(tracer)
                    .with_filter(LevelFilter::DEBUG),
            )
            .try_init()
    } else {
        registry.try_init()
    }
    .expect("failed to register tracer with registry");
}

fn init_jaeger_tracer(agent_endpoint: &str) -> Result<Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_agent_endpoint(agent_endpoint)
        .with_service_name("verification")
        .with_auto_split_batch(true)
        .install_batch(opentelemetry::runtime::Tokio)
}
