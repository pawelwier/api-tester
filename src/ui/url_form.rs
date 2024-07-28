use eframe::egui::{Id, Response, TextEdit, Ui};

fn get_address_bar(ui: &mut Ui, text: &mut String) -> Id {
    let text_edit = TextEdit::singleline(text);
    let id = Id::new("new_address_bar_id");
    text_edit
        .id(id)
        .return_key(None)
        .hint_text("Web address ...")
        .show(ui);
    id
}

fn get_go_button(ui: &mut Ui) -> Response {
    ui.button("Go!")
}

pub fn get_url_form(ui: &mut Ui, text: &mut String) -> (Id, Response) {
    (
        get_address_bar(ui, text),
        get_go_button(ui)
    )
}