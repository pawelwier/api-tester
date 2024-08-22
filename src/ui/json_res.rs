use eframe::egui::{
    Label, Ui
};
use serde_json::{
    to_string_pretty, Value
};

pub fn get_json_res_text(ui: &mut Ui, json: &Option<Value>) {
    if let Some(json_option) = json {
        if let Some(json_body) = json_option.as_object() {
            let text: String = match to_string_pretty(json_body) {
                Ok(data) => data,
                Err(_) => "Result is not a valid JSON.".to_owned()
            };
            ui.add(Label::new(text));
        }
    }
}
