use std::sync::mpsc::{Receiver, Sender};

use eframe::{
    egui::{
        CentralPanel, Context
    },
    App, CreationContext, Frame
};
use reqwest::{get, header, Response, StatusCode};

use crate::ui::{
    address_bar::get_address_bar,
    go_button::get_go_button,
    main_header::get_main_header
};

pub struct BrowserApp {
    sender: Sender<Response>,
    receiver: Receiver<Response>,
    pub address_text: String
}

pub struct HttpResponse {
    status: StatusCode,
    headers: header::HeaderMap
}

impl Default for BrowserApp {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self { 
            sender,
            receiver,
            address_text: "".to_owned() 
        }
    }
}

fn send_request(address: String, sender: Sender<Response>, ctx: Context) {
    tokio::spawn(async move {
        let response: Response = get(address)
            .await
            .expect("Unable to send request");
        let _ = sender.send(response);
        ctx.request_repaint();
    });
}

fn get_http_response(response: Response) -> HttpResponse {
    let status = get_status(&response);
    let headers = get_headers(&response);

    HttpResponse {
        status,
        headers
    }
}

fn get_status(response: &Response) -> StatusCode {
    response
        .status()
}

fn get_headers(response: &Response) -> header::HeaderMap {
    response
        .headers()
        .clone()
}

/*
    async fn get_html(response: Response) -> String {
        response
            .text()
            .await
            .expect("Unable to get html response")
    }

    async fn get_json(response: Response) -> String {
        response
            .json()
            .await
            .expect("Unable to get json response")
    }
*/

impl BrowserApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        // TODO: set options for cc
        Self::default()
    }
}

impl App for BrowserApp {
    fn update(
        &mut self,
        ctx: &Context,
        _frame: &mut Frame
    ) {
        if let Ok(response) = self.receiver.try_recv() {
            let http_response = get_http_response(response);        
            let status = http_response.status;
            let headers = http_response.headers;

            println!("status: {}\n", status);
            println!("headers:");
            for (key, value) in headers.iter() {
                println!("{:?}: {:?}", key, value);
            }
            println!("\n\n");
        }
        
        CentralPanel::default().show(ctx, |ui| {
            get_main_header(ui);
            get_address_bar(ui, &mut self.address_text);
            let go_btn = get_go_button(ui);
            
            if go_btn.clicked() {
                send_request(self.address_text.to_string(), self.sender.clone(), ctx.clone());
            }
        });
    }
}