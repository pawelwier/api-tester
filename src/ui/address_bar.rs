use eframe::egui::{TextEdit, Ui};

pub fn get_address_bar(ui: &mut Ui, text: &mut String) -> () {
    TextEdit::singleline(text)
        .hint_text("Web address ...")
        .show(ui);
}