use std::path::PathBuf;
use tracing::{Level, dispatcher, info};
use tracing_subscriber::{EnvFilter, Layer, filter, fmt, layer::SubscriberExt};

pub fn setup_logger(path: &PathBuf) -> dispatcher::DefaultGuard {
    let layer_stdout = fmt::Layer::default()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_default_env());

    if let Some(parent_dir) = path.parent() {
        if !parent_dir.try_exists().unwrap() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }
    }

    let trace_log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
    let layer_trace = fmt::Layer::default()
        .with_writer(trace_log_file)
        .without_time()
        .with_level(false)
        .with_target(false)
        .with_filter(filter::filter_fn(|metadata| {
            matches!(*metadata.level(), Level::TRACE)
        }));

    let registry = tracing_subscriber::registry()
        .with(layer_stdout)
        .with(layer_trace);

    let guard = tracing::subscriber::set_default(registry);

    info!("Logger initialized");

    guard
}
