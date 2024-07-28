use eframe::egui::{Label, Ui};
use reqwest::header::HeaderMap;

pub fn get_headers_text(ui: &mut Ui, header_map: &Option<HeaderMap>) {
    let mut headers_text: String = "Headers:\n".to_string();

    if let Some(headers) = header_map {
        for (key, value) in headers.iter() {
            headers_text += &format!("{:?}: {:?}\n", key, value);
        }
        
        ui.add(Label::new(headers_text));
    }
}
