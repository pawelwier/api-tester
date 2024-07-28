use std::sync::mpsc::Sender;

use eframe::egui::Context;

use reqwest::{
    Error, get, header, Response, StatusCode
};

use super::HttpResponse;

pub fn send_request(address: String, sender: Sender<Result<Response, Error>>, ctx: Context) {
    tokio::spawn(async move {
        let response_result: Result<Response, reqwest::Error> = get(address).await;
        let _ = sender.send(response_result);
        ctx.request_repaint();
    });
}

pub fn get_http_response(response: Response) -> HttpResponse {
    let status = get_status(&response);
    let headers = get_headers(&response);

    HttpResponse {
        status,
        headers
    }
}

pub fn get_status(response: &Response) -> StatusCode {
    response
        .status()
}

pub fn get_headers(response: &Response) -> header::HeaderMap {
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
