use eframe::{egui::ViewportBuilder, NativeOptions};
use browser_app::BrowserApp;

mod browser_app;

fn main() {
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


