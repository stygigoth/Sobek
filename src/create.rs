use iced::{Length, pure::{Element, widget::{Container, Button}}};
use iced_aw::TabLabel;
use crate::SobekMsg;

pub struct CreateTab;

impl CreateTab {
    pub fn new () -> Self {
        CreateTab
    }

    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text(String::from("Create"))
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let create = Button::new("Create");

        Container::new(create)
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill)
        .padding(200).into()
    }
}