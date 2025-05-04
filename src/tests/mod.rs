use std::fmt;
use std::path::PathBuf;
use tracing::Event;
use tracing::Subscriber;
use tracing::dispatcher;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::filter::FilterFn;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::format;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;

struct DoctorEventFormatter;

impl<S, N> FormatEvent<S, N> for DoctorEventFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

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

pub fn logging_layer_gameboy_doctor<S>(path: &PathBuf) -> Box<dyn Layer<S> + Send + Sync>
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
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

    tracing_subscriber::fmt::layer()
        .with_writer(trace_log_file)
        .without_time()
        .with_level(false)
        .with_target(false)
        .event_format(DoctorEventFormatter)
        .with_filter(FilterFn::new(|metadata| metadata.name() == "cpu::state"))
        .boxed()
}

pub fn setup_default_logger() -> dispatcher::DefaultGuard {
    let registry = tracing_subscriber::registry().with(logging_layer_stdout());

    let guard = tracing::subscriber::set_default(registry);

    info!("Logger initialized");

    guard
}

pub fn setup_gameboy_doctor_logger(path: &PathBuf) -> dispatcher::DefaultGuard {
    let registry = tracing_subscriber::registry()
        .with(logging_layer_stdout())
        .with(logging_layer_gameboy_doctor(path));

    let guard = tracing::subscriber::set_default(registry);

    info!("Logger initialized");

    guard
}
