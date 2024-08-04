use eframe::{
    egui::{
        Color32, Context, Ui, Key::Enter
    }, 
    CreationContext
};
use reqwest::Error;

use crate::{
    http::{
        req_res::send_request, HttpResponse, HttpStatus
    }, 
    ui::{
        status_text::get_status_color, url_form::get_url_form
    }
};

use super::browser_app::BrowserApp;

impl BrowserApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        // TODO: set options for cc ?
        Self::default()
    }

    pub fn set_status(
        &mut self,
        text: String,
        code: Option<u16>,
        color: Option<Color32>
    ) {
        self.status = HttpStatus {
            code,
            text,
            color
        }
    }

    pub fn set_valid_data(
        &mut self,
        http_response: HttpResponse
    ) {
        let status = http_response.status;
        self.set_status(
            status.canonical_reason().unwrap().to_string(),
            Some(status.as_u16()),
            get_status_color(status)
        );
        self.headers = Some(http_response.headers);
        self.json = http_response.json;
    }

    pub fn set_invalid_data(&mut self) {
        self.set_status(
            "Unable to send request".to_string(),
            None,
            Some(Color32::WHITE)
        );
        self.headers = None;
    }

    pub fn set_data(
        &mut self,
        response_option: Result<HttpResponse, Error>
    ) {
        match response_option {
            Ok(http_response) => {
                self.set_valid_data(http_response);
            },
            Err(_) => { self.set_invalid_data(); }
        }
    }

    pub fn handle_send_req(
        &mut self,
        ctx: &Context,
        ui: &mut Ui
    ) {
        let mut is_send_req: bool = false;
        let (address_bar_id, go_btn) = get_url_form(ui, &mut self.address_text);
        let focused_id = ctx.memory(|i| i.focused());
        match focused_id {
            Some(id) => { if address_bar_id == id {
                ctx.input(|i| { if i.key_down(Enter) {
                    is_send_req = true;
                } });
            } },
            None => {}
        }
        
        if go_btn.clicked() { is_send_req = true; };
        
        if is_send_req {
            send_request(
                self.address_text.to_string(),
                self.sender.clone(),
                self.method,
                ctx.clone()
            );
        }
    }
}