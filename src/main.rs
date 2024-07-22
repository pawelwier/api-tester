mod browser_app;

use browser_app::BrowserApp;

fn main() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native(
        "Web browser", 
        native_options, 
        Box::new(
            |cc| Ok(Box::new(BrowserApp::new(cc)))
        )
    );
}


