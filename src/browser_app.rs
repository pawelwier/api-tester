use std::sync::mpsc::{
    Receiver, Sender
};

use eframe::{
    egui::{
        CentralPanel, Color32, Context
    },
    App, CreationContext, Frame
};
use reqwest::{
    header::HeaderMap, Error, Response
};

use crate::ui::{
    address_bar::get_address_bar, go_button::get_go_button, headers::get_headers_text, main_header::get_main_header, status_text::{
        get_status_color, get_status_text
    }
};

use crate::http::{
    HttpStatus,
    req_res::{
        send_request, get_http_response
    }
};

pub struct BrowserApp {
    sender: Sender<Result<Response, Error>>,
    receiver: Receiver<Result<Response, Error>>,
    pub address_text: String,
    pub status: HttpStatus,
    pub headers: Option<HeaderMap>,
}

impl BrowserApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        // TODO: set options for cc
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
}

impl Default for BrowserApp {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self { 
            sender,
            receiver,
            address_text: "".to_owned(),
            status: HttpStatus { 
                text: "".to_owned(),
                code: None,
                color: None
            },
            headers: None
        }
    }
}

impl App for BrowserApp {
    fn update(
        &mut self,
        ctx: &Context,
        _frame: &mut Frame
    ) {
        if let Ok(response) = self.receiver.try_recv() {
            if let Ok(res) = response {
                let http_response = get_http_response(res);
                let status = http_response.status;
                self.set_status(
                    status.canonical_reason().unwrap().to_string(),
                    Some(status.as_u16()),
                    get_status_color(status)
                );
                self.headers = Some(http_response.headers);
            } else {
                self.set_status(
                    "Unable to send request".to_string(),
                    None,
                    Some(Color32::WHITE)
                );
                self.headers = None;
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            get_main_header(ui);
            get_address_bar(ui, &mut self.address_text);
            let go_btn = get_go_button(ui);
            
            if go_btn.clicked() {
                send_request(self.address_text.to_string(), self.sender.clone(), ctx.clone());
            }

            get_status_text(ui, &self.status);
            get_headers_text(ui, &self.headers);
        });
    }
}