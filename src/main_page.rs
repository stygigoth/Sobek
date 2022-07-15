use iced::{Length, Padding, Alignment, pure::{Element, widget::{Container, Column, Row, Button, TextInput}}};
use crate::SobekMsg;

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
        let file_dialog = Button::new("Select Resources Directory").on_press(SobekMsg::SelectDir);
        let conf = Button::new("Confirm").on_press(SobekMsg::ConfirmMID);
        
        let row = Row::new().push(file_dialog).push(conf).spacing(20).align_items(Alignment::Center);
        let col = Column::new().push(input).push(row).spacing(10);

        Container::new(col)
            .center_x().center_y()
            .width(Length::Fill).height(Length::Fill)
            .padding(Padding::new(200))
            .into()
    }
}
