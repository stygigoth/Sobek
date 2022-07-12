use iced::{Length, Padding, pure::{Element, widget::{Container, Row, Column, Button, TextInput, Checkbox}}};
use crate::{SobekMsg, Views};

pub struct SimpleBlockPage {
    pub id: String,
    pub has_bi: bool,
    pub drops_self: bool
}

impl SimpleBlockPage {
    pub fn new() -> Self {
        SimpleBlockPage {
            id: String::from(""),
            has_bi: false,
            drops_self: false
        }
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let input = TextInput::new("Block ID", &self.id, SobekMsg::ChangeBID).padding(10);

        let has_bi = Checkbox::new(self.has_bi, "Has BlockItem?", SobekMsg::ToggleBI);
        let drops_self = Checkbox::new(self.drops_self, "Drops Self?", SobekMsg::ToggleDS);
        let create = Button::new("Create").on_press(SobekMsg::ConfirmSimple);

        let row = Row::new().push(has_bi).push(drops_self).push(create).spacing(20);
        let col = Column::new().push(input).push(row).spacing(10);
        Container::new(col)
            .center_x().center_y()
            .width(Length::Fill).height(Length::Fill)
            .padding(Padding::new(200))
            .into()
    }
}
