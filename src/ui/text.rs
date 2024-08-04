use eframe::egui::RichText;

fn get_text(
    text: impl Into<String>,
    size: f32
) -> RichText {
    RichText::new(text).size(size)
}

pub fn get_regular_text(text: impl Into<String>) -> RichText {
    get_text(text, 20.0)
}

pub fn get_big_text(text: impl Into<String>) -> RichText {
    get_text(text, 30.0)
}