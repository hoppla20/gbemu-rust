use tracing::dispatcher;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;

pub fn logging_layer_stdout<S>() -> Box<dyn Layer<S> + Send + Sync>
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_default_env())
        .boxed()
}

pub fn setup_default_logger() -> dispatcher::DefaultGuard {
    let registry = tracing_subscriber::registry().with(logging_layer_stdout());

    let guard = tracing::subscriber::set_default(registry);

    info!("Logger initialized");

    guard
}
