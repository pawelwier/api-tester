use eframe::egui::Color32;
use reqwest::{
    header, StatusCode
};

pub mod req_res;

pub struct HttpStatus {
    pub text: String,
    pub code: Option<u16>,
    pub color: Option<Color32>,
}
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: header::HeaderMap
}