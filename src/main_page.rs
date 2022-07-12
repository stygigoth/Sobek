use iced::{Length, Padding, pure::{Element, widget::{Container, Column, Button, TextInput}}};
use crate::{SobekMsg, Views};

pub struct MainPage {
    pub mod_id: String
}

impl MainPage {
    pub fn new() -> Self {
        MainPage {
            mod_id: String::from("")
        }
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let input = TextInput::new("Mod ID", &self.mod_id, SobekMsg::ChangeMID).padding(10);
        let conf = Button::new("Confirm").on_press(SobekMsg::ConfirmMID);
        let col = Column::new().push(input).push(conf).spacing(10);

        Container::new(col)
            .center_x().center_y()
            .width(Length::Fill).height(Length::Fill)
            .padding(Padding::new(200))
            .into()
    }
}
