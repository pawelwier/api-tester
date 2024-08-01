use std::sync::mpsc::Sender;

use eframe::egui::Context;

use reqwest::{
    header, Error, RequestBuilder, Response, StatusCode
};

use crate::browser_app::ReqMethod;

use super::HttpResponse;

fn get_address_url(address: String) -> String {
    let prefix = "https://".to_owned();
    if !address.starts_with(&prefix) {
        prefix + &address
    } else { address }
}

fn get_req_builder(
    method: ReqMethod,
    client: reqwest::Client,
    url: String
) -> RequestBuilder {
    match method {
        ReqMethod::Get => client.get(&url),
        ReqMethod::Post => client.post(&url),
        ReqMethod::Put => client.put(&url),
        ReqMethod::Delete => client.delete(&url),
    }
}

pub fn send_request(
    address: String,
    sender: Sender<Result<Response, Error>>,
    method: ReqMethod,
    ctx: Context
) {
    let client = reqwest::Client::new();
    let url = get_address_url(address);


    tokio::spawn(async move {
        let req_builder = get_req_builder(
            method, 
            client,
            url
        );
        let response_result: Result<Response, reqwest::Error> = req_builder
            .send()
            .await;
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
