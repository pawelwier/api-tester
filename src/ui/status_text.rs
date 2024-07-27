use eframe::egui::{Label, Ui};

pub fn get_status_text(ui: &mut Ui, text: &String) -> () {
    if text.len() < 1 { return }
    ui.add(Label::new(text));
}