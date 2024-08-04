use eframe::egui::Ui;

use crate::app::{
    browser_app::BrowserApp,
    response_tabs::ResponseTabType
};

use super::{
    layouts::horizontal_align,
    text::get_regular_text
};

// TODO: adjust to > 2 options
fn switch_tab_type(app: &BrowserApp) -> ResponseTabType {
    if app.response_tabs.visible_tab == ResponseTabType::Headers { 
        ResponseTabType::Json
    } else {
        ResponseTabType::Headers
    }
}

pub fn get_response_tab_select(ui: &mut Ui, app: &mut BrowserApp) -> () {
    ui.with_layout(horizontal_align(), |ui| {
        let tabs = &app.response_tabs.tabs;
        for tab_info in tabs {
            // TODO: hide tab whose value is None
             if ui.selectable_value(
                &mut &app.response_tabs.visible_tab,
                &tab_info.tab_type,
                get_regular_text(&tab_info.name)
            ).clicked() {   
                app.response_tabs.visible_tab = switch_tab_type(app);
            }
        }
    });
    ui.add_space(10.);
}