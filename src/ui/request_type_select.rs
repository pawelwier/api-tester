use eframe::egui::Ui;

use crate::app::browser_app::{
    BrowserApp, ReqMethod
};

fn get_req_method_name(method: &ReqMethod) -> String {
    match method {
        ReqMethod::Get => "Get".to_owned(),
        ReqMethod::Post => "Post".to_owned(),
        ReqMethod::Put => "Put".to_owned(),
        ReqMethod::Delete => "Delete".to_owned()
    }
}

pub fn get_req_method_select(ui: &mut Ui, app: &mut BrowserApp) -> () {
    for method in ReqMethod::VALUES {
        let text = get_req_method_name(&method.clone());
        ui.selectable_value(&mut app.method, method, text);
    }
}