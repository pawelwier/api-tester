use std::sync::mpsc::{Receiver, Sender};

use eframe::{
    egui::{
        CentralPanel, Context
    },
    App, CreationContext, Frame
};

use crate::ui::{
    address_bar::get_address_bar,
    go_button::get_go_button,
    main_header::get_main_header
};

pub struct BrowserApp {
    sender: Sender<String>,
    receiver: Receiver<String>,
    pub address_text: String
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

fn send_request(address: String, sender: Sender<String>, ctx: Context) {
    tokio::spawn(async move {
        let html: String = reqwest::get(address)
            .await
            .expect("Unable to send request")
            .text()
            .await
            .expect("Unable to get html response");

        let _ = sender.send(html);
        ctx.request_repaint();
    });
}

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
        if let Ok(html_response) = self.receiver.try_recv() {
            // TODO: display html
            println!("addr: {}", html_response);
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