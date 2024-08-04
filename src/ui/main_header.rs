use eframe::egui::Ui;

use super::text::get_big_text;

pub fn get_main_header(ui: &mut Ui) -> () {
    ui.heading(get_big_text("Test your url"));
    ui.add_space(20.);
}