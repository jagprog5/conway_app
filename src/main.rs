#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let size = Some(eframe::epaint::Vec2 { x: 275.0, y: 275.0 });
    let options = eframe::NativeOptions {
        min_window_size: size,
        max_window_size: size,
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Conway App!",
        options,
        Box::new(|cc| Box::new(conway_app::ConwayApp::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(conway_app::ConwayApp::new(cc))),
    )
    .expect("failed to start eframe");
}
