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
    header::HeaderMap, Error
};

use crate::{http::HttpResponse, ui::{
    headers::get_headers_text,
    json_res::get_json_res_text,
    main_header::get_main_header,
    request_type_select::get_req_method_select,
    status_text::{
        get_status_color, get_status_text
    },
    url_form::get_url_form
}};

use crate::http::{
    HttpStatus,
    req_res::send_request
};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum ReqMethod {
    Get,
    Post,
    Put,
    Delete,
    // Patch
}

impl ReqMethod {
    pub const VALUES: [Self; 4] = [
        ReqMethod::Get, ReqMethod::Post, ReqMethod::Put, ReqMethod::Delete
    ];
}

pub struct BrowserApp {
    sender: Sender<Result<HttpResponse, Error>>,
    receiver: Receiver<Result<HttpResponse, Error>>,
    
    pub method: ReqMethod,

    pub address_text: String,
    pub status: HttpStatus,
    pub headers: Option<HeaderMap>,
    pub json: Option<serde_json::Value>
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
            method: ReqMethod::Get,
            headers: None,
            json: None
        }
    }
}

// TODO: Refactor
impl App for BrowserApp {
    fn update(
        &mut self,
        ctx: &Context,
        _frame: &mut Frame
    ) {
        if let Ok(response) = self.receiver.try_recv() {
            if let Ok(http_response) = response {
                let status = http_response.status;
                self.set_status(
                    status.canonical_reason().unwrap().to_string(),
                    Some(status.as_u16()),
                    get_status_color(status)
                );
                self.headers = Some(http_response.headers);
                self.json = http_response.json;
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
            get_req_method_select(ui, self);

            let mut is_send_req: bool = false;
            let (address_bar_id, go_btn) = get_url_form(ui, &mut self.address_text);
            let focused_id = ctx.memory(|i| i.focused());
            match focused_id {
                Some(id) => { if address_bar_id == id {
                    ctx.input(|i| { if i.key_down(eframe::egui::Key::Enter) {
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

            get_status_text(ui, &self.status);
            get_headers_text(ui, &self.headers);
            get_json_res_text(ui, &self.json);
        });
    }
}