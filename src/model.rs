use iced::{Length, pure::{Element, widget::{Container, Text, Button, Column}}};
use iced_aw::{TabLabel, pure::Split};
use crate::SobekMsg;

pub struct ModelTab {
    pub split: u16
}

impl ModelTab {
    pub fn new () -> Self {
        ModelTab {
            split: 256
        }
    }

    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text(String::from("Model"))
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let first = Container::new(Text::new("First"))
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill);
        let second = Container::new(Text::new("Second"))
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill);
        let spl = Split::new(first, second, Option::from(self.split), iced_aw::pure::split::Axis::Vertical, SobekMsg::ModelSplitSize);

        let import = Button::new("Import Model from BlockBench");
        let con = Container::new(import).center_x().center_y().width(Length::Fill);

        let col = Column::new().push(con).push(spl).padding(10).spacing(10);

        Container::new(col).into()
    }
}