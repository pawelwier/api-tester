use eframe::egui::{
    Label, Ui
};
use serde_json::to_string_pretty;

pub fn get_json_res_text(ui: &mut Ui, json: &Option<serde_json::Value>) {
    if let Some(json_option) = json {
        if let Some(json_body) = json_option.as_object() {
            let text: String;
            match to_string_pretty(json_body) {
                Ok(json_value) => { text = json_value },
                Err(_) => { text = "Result is not a valid JSON.".to_owned() }
            }
            ui.add(Label::new(text));
        }
    }
}
