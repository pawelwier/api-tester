use eframe::egui::{
    Label, Ui
};

pub fn get_json_res_text(ui: &mut Ui, json: &Option<serde_json::Value>) {
    let mut json_text: String = "".to_string();

    // TODO: parse to be readable
    if let Some(json_option) = json {
        if let Some(json_body) = json_option.as_object() {
            for (key, value) in json_body {
                json_text += &format!("{:?}: {:?}\n", key, value);
            }
        }
        
        ui.add(Label::new(json_text));
    }
}
