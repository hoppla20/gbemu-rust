use tracing::Level;
use tracing::info;
use tracing_subscriber::{
    EnvFilter, Layer, filter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

pub fn setup_logger() {
    let layer_stdout = fmt::Layer::default()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_default_env());

    let trace_log = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("trace.log")
        .unwrap();
    let layer_trace = fmt::Layer::default()
        .with_writer(trace_log)
        .without_time()
        .with_level(false)
        .with_target(false)
        .with_filter(filter::filter_fn(|metadata| {
            matches!(*metadata.level(), Level::TRACE)
        }));

    tracing_subscriber::registry()
        .with(layer_stdout)
        .with(layer_trace)
        .init();

    info!("Initialized tracing logger");
}
