#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    warnings,
    future_incompatible,
    nonstandard_style,
    non_ascii_idents,
    clippy::restriction,
    rust_2018_compatibility,
    rust_2021_compatibility,
    unused
)]
#![allow(
    clippy::print_stdout,
    clippy::use_debug,
    clippy::missing_docs_in_private_items,
    clippy::absolute_paths,
)]
#![allow(clippy::blanket_clippy_restriction_lints)] //workaround clippy
#![allow(clippy::needless_return)]
// might want to deny later:
#![allow(clippy::default_numeric_fallback)] // might want to deny later!
#![allow(clippy::dbg_macro)]
//#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    return eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| return Box::new(eframe_template::TemplateApp::new(cc))),
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

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
