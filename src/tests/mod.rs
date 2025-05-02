use log::info;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    Layer,
    filter::{self, LevelFilter},
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub use super::prelude::*;

pub fn setup_logger() -> Vec<WorkerGuard> {
    let mut guards = vec![];

    let trace_log = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("trace.log")
        .unwrap();
    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
    let layer_stdout = fmt::Layer::default()
        .with_writer(non_blocking)
        .with_filter(LevelFilter::DEBUG);
    guards.push(guard);

    let (non_blocking, guard) = tracing_appender::non_blocking(trace_log);
    let layer_trace = fmt::Layer::default()
        .with_writer(non_blocking)
        .without_time()
        .with_level(false)
        .with_target(false)
        .with_filter(filter::filter_fn(|metadata| {
            matches!(*metadata.level(), Level::TRACE)
        }));
    guards.push(guard);

    tracing_subscriber::registry()
        .with(layer_stdout)
        .with(layer_trace)
        .init();

    info!("Initialized tracing logger");

    guards
}
