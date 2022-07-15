use iced::{Length, pure::{Element, widget::{Container, Column, TextInput, Button}}};
use iced_aw::TabLabel;
use crate::SobekMsg;

pub struct CreateTab {
    pub block_id: String
}

impl CreateTab {
    pub fn new () -> Self {
        CreateTab {
            block_id: String::from("")
        }
    }

    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text(String::from("Create"))
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let text = TextInput::new("Block ID", &self.block_id, SobekMsg::ChangeAdvBID).padding(10);
        let create = Button::new("Create").on_press(SobekMsg::Create);

        Container::new(Column::new().spacing(20).push(text).push(create))
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill)
        .padding(200).into()
    }
}