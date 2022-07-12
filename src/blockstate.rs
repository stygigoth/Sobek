use iced::{Length, pure::{Element, widget::{Container, Text}}};
use iced_aw::{TabLabel, pure::Split};
use crate::SobekMsg;

pub enum Blockstate {
    Variants,
    Multipart
}

pub struct BlockstateTab {
    blockstate: Blockstate,
    pub split: u16
}

impl BlockstateTab {
    pub fn new () -> Self {
        BlockstateTab {
            blockstate: Blockstate::Variants,
            split: 256
        }
    }

    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text(String::from("Blockstate"))
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let first = Container::new(Text::new("First"))
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill);
        let second = Container::new(Text::new("Second"))
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill);

        Split::new(first, second, Option::from(self.split), iced_aw::pure::split::Axis::Vertical, SobekMsg::BlockstateSplitSize).into()
    }
}