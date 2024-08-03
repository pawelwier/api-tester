use eframe::{
    egui::ViewportBuilder, NativeOptions
};
use app::browser_app::BrowserApp;
use std::time::Duration;
use tokio::runtime::Runtime;

mod app;
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

    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([
            600.0, 700.0
        ]),
        centered: true,
        ..NativeOptions::default()
    };

    let _ = eframe::run_native(
        "Api tester",
        options,
        Box::new(
            |cc| Ok(Box::new(BrowserApp::new(cc)))
        )
    );
}


