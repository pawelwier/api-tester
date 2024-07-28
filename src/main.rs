use eframe::{
    egui::ViewportBuilder, NativeOptions
};
use browser_app::BrowserApp;
use std::time::Duration;
use tokio::runtime::Runtime;

mod browser_app;
mod ui;
mod http;

fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();
    
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([
            1200.0, 800.0
        ]),
        ..NativeOptions::default()
    };

    let _ = eframe::run_native(
        "Web browser",
        native_options,
        Box::new(
            |cc| Ok(Box::new(BrowserApp::new(cc)))
        )
    );
}


