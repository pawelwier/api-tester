use eframe::{egui, App};

#[derive(Default)]
pub struct BrowserApp {}

impl BrowserApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // TODO: set options for cc
        Self::default()
    }
}

impl App for BrowserApp {
    fn update(
        &mut self, 
        ctx: &egui::Context, 
        _frame: &mut eframe::Frame
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let header = ui.heading("I will be a web browser");
            let btn = ui.button("test");

            if btn.clicked() {
                header.highlight();
            }
        });
    }
}