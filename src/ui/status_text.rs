use std::collections::HashMap;

use eframe::egui::{
    Ui, Color32
};
use reqwest::StatusCode;

use crate::http::HttpStatus;

pub fn get_status_text(ui: &mut Ui, status: &HttpStatus) -> () {
    let has_code: bool = status.code.is_some();

    let text = if has_code {
        format!("{} : {:?}", &status.text, status.code.unwrap())
    } else { status.text.to_string() };

    let color = if status.color.is_none() {
        Color32::WHITE
    } else { status.color.unwrap() };
        
    ui.colored_label(color, &text);
}

pub fn get_status_color(status: StatusCode) -> Option<Color32> {
    let color_map = HashMap::from([
        (status.is_informational(), Color32::BLUE),
        (status.is_success(), Color32::GREEN),
        (status.is_redirection(), Color32::BLUE),
        (status.is_client_error(), Color32::RED),
        (status.is_server_error(), Color32::RED)
    ]);

    let match_color = color_map.get(&true);

    Some(*match_color.unwrap_or(&Color32::WHITE))
}