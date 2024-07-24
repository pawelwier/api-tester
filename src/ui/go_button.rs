use eframe::egui::{Response, Ui};

pub fn get_go_button(ui: &mut Ui) -> Response {
    ui.button("Go!")
}