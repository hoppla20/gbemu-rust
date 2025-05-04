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

pub fn setup_logger(path: &PathBuf) -> dispatcher::DefaultGuard {
    let layer_stdout = tracing_subscriber::fmt::Layer::default()
        .pretty()
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
    let layer_trace = tracing_subscriber::fmt::Layer::default()
        .with_writer(trace_log_file)
        .without_time()
        .with_level(false)
        .with_target(false)
        .event_format(DoctorEventFormatter)
        .with_filter(FilterFn::new(|metadata| metadata.name() == "cpu::state"));

    let registry = tracing_subscriber::registry()
        .with(layer_stdout)
        .with(layer_trace);

    let guard = tracing::subscriber::set_default(registry);

    info!("Logger initialized");

    guard
}
