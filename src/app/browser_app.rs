use std::sync::mpsc::{
    channel, Receiver, Sender
};

use eframe::{
    egui::{
        CentralPanel, Context
    },
    App, Frame
};
use reqwest::{
    header::HeaderMap, Error
};

use crate::{
    http::HttpResponse, 
    ui::{
        headers::get_headers_text, 
        json_res::get_json_res_text,
        main_header::get_main_header,
        request_type_select::get_req_method_select,
        response_tab_select::get_response_tab_select,
        status_text::get_status_text
    }
};

use crate::http::HttpStatus;

use super::response_tabs::ResponseTabs;

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
    pub sender: Sender<Result<HttpResponse, Error>>,
    pub receiver: Receiver<Result<HttpResponse, Error>>,
    
    pub method: ReqMethod,

    pub address_text: String,
    pub status: HttpStatus,
    pub headers: Option<HeaderMap>,
    pub json: Option<serde_json::Value>,

    pub response_tabs: ResponseTabs
}

impl Default for BrowserApp {
    fn default() -> Self {
        let (sender, receiver) = channel();

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
            json: None,
            response_tabs: Default::default()
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
            self.set_data(response);
        }

        CentralPanel::default().show(ctx, |ui| {
            get_main_header(ui);
            get_req_method_select(ui, self);
            self.handle_send_req(ctx, ui);
            get_status_text(ui, &self.status);
            if self.show_response_tabs() { get_response_tab_select(ui, self); }
            if self.show_headers() { get_headers_text(ui, &self.headers) };
            if self.show_json() { get_json_res_text(ui, &self.json) };
        });
    }
}