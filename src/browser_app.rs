use eframe::{
    egui::{
        CentralPanel, Context
    },
    App, CreationContext, Frame
};

#[derive(Default)]
pub struct BrowserApp {}

impl BrowserApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        // TODO: set options for cc
        Self::default()
    }
}

impl App for BrowserApp {
    fn update(
        &mut self,
        ctx: &Context,
        _frame: &mut Frame
    ) {
        CentralPanel::default().show(ctx, |ui| {
            let header = ui.heading("I will be a web browser");
            let btn = ui.button("test");

            if btn.clicked() {
                header.highlight();
            }
        });
    }
}