use eframe::egui::{
    Label, Ui
};
use reqwest::header::HeaderMap;
use serde_json::{
    Map, from_str, to_string_pretty, to_value, Value
};

pub fn get_headers_text(ui: &mut Ui, header_map: &Option<HeaderMap>) {
    let mut result_json = Map::new();

    if let Some(headers) = header_map {
        for (key, value) in headers.iter() {
            if let Ok(val) = value.to_str() {
                let json_result = from_str::<Value>(val);

                let json_value = match json_result {
                    Ok(data) => data,
                    Err(_) => to_value(val).unwrap()
                };

                result_json.insert(key.to_string(), json_value);
            }
        }

        let text = match to_string_pretty(&result_json) {
            Ok(data) => data,
            Err(_) => "Result is not a valid JSON".to_owned()
        };
        
        ui.add(Label::new(text));
    }
}
