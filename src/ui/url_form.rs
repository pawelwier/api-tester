use eframe::egui::{
    Align, Button, CursorIcon, FontSelection, Id, Response, TextEdit, TextStyle, Ui
};

use super::{
    layouts::horizontal_align,
    text::get_regular_text
};

fn get_address_bar(ui: &mut Ui, text: &mut String) -> Id {
    let text_edit = TextEdit::singleline(text)
        .font(FontSelection::Style(TextStyle::Heading));
    let id = Id::new("new_address_bar_id" /* ? */);
    ui
        .add_sized(
            [400., 25.],
            text_edit
                .id(id)
                .return_key(None)
                .hint_text(get_regular_text("Web address ..."))
                .vertical_align(Align::Center)
            );
    id
}

fn get_go_button(ui: &mut Ui) -> Response {
    let text = get_regular_text("Go!");
    ui
        .add_sized(
            [60., 25.],
            Button::new(text)
        )
        .on_hover_cursor(CursorIcon::PointingHand)
}

pub fn get_url_form(ui: &mut Ui, text: &mut String) -> (Id, Response) {
    let mut id: Id = Id::NULL;
    let mut response_option: Option<Response> = None;
    ui.with_layout(horizontal_align(), |ui| {
        id = get_address_bar(ui, text);
        response_option = Some(get_go_button(ui));
        
    });
    ui.add_space(10.);
    (id, response_option.unwrap())
}