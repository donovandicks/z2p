use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Compose layers into a `tracing` subscriber
///
/// # Args
///
/// * `name` - The name of application, to be included in log information
/// * `env_filter` - The minimum log level to report on
///
/// # Returns
///
/// * An implementor of `Subscriber` that describes how logs should be filtered
/// and formatted
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    // Fallback to >= info level if the RUST_LOG variable has not been set
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Initializes a `tracing` subscriber as a global default for processing spans
///
/// # Args
///
/// * `subscriber` - An implementor of `Subscribe`
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
