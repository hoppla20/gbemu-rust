// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use app::GbemuApp;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use gbemu_rust_lib::prelude::Emulator;
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .pretty()
        .with_writer(std::io::stdout)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let native_options = eframe::NativeOptions {
        // TODO: icons
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 400.0])
            .with_min_inner_size([400.0, 400.0]),
        ..eframe::NativeOptions::default()
    };

    let args = Args::parse();
    let mut emulator: Option<Emulator> = None;

    if let Some(file_path) = args.file_path.as_deref() {
        match std::fs::read(file_path) {
            Ok(rom) => {
                emulator = Some(Emulator::new_from_buffer(rom, true, None, None).unwrap());
            },
            Err(err) => {
                log::error!("{}", err);
                todo!("Handle error if file read is not successful!");
            },
        }
    }

    eframe::run_native(
        "gbemu",
        native_options,
        Box::new(|cc| Ok(Box::new(GbemuApp::new(&cc, emulator).unwrap()))),
    )
    .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::fmt::format::Pretty;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_web::{MakeWebConsoleWriter, performance_layer};

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new());
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .with(LevelFilter::WARN)
        .init();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("gbemu_rust_app_main")
            .expect("Failed to find gbemu_rust_app_main")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("gbemu_rust_app_main was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(GbemuApp::new(&cc, None).unwrap()))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                },
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                },
            }
        }
    });
}
