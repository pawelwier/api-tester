use std::sync::mpsc::{Receiver, Sender};

use eframe::{
    egui::{
        CentralPanel, Context
    },
    App, CreationContext, Frame
};
use reqwest::{
    Error, get, header, Response, StatusCode
};

use crate::ui::{
    address_bar::get_address_bar,
    go_button::get_go_button,
    main_header::get_main_header,
    status_text::get_status_text
};

pub struct BrowserApp {
    sender: Sender<Result<Response, Error>>,
    receiver: Receiver<Result<Response, Error>>,
    pub address_text: String,
    pub status_text: String, // TODO: change to StatusCode
    pub headers_text: String,
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
            address_text: "".to_owned(),
            status_text: "".to_owned(),
            headers_text: "".to_owned()
        }
    }
}

fn send_request(address: String, sender: Sender<Result<Response, Error>>, ctx: Context) {
    tokio::spawn(async move {
        let response_result: Result<Response, reqwest::Error> = get(address).await;
        let _ = sender.send(response_result);
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
            if let Ok(res) = response {
                let http_response = get_http_response(res);        
                self.status_text = http_response.status.to_string();
                let headers = http_response.headers;
    
    
    
                println!("status: {}\n", self.status_text);
                println!("headers:");
                for (key, value) in headers.iter() {
                    println!("{:?}: {:?}", key, value);
                }
                println!("\n\n");
            } else {
                self.status_text = "Unable to send request".to_string();
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            get_main_header(ui);
            get_address_bar(ui, &mut self.address_text);
            let go_btn = get_go_button(ui);
            
            if go_btn.clicked() {
                send_request(self.address_text.to_string(), self.sender.clone(), ctx.clone());
            }

            get_status_text(ui, &self.status_text);
        });
    }
}