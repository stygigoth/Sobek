use iced::{Length, Padding, pure::{Element, widget::{Container, Row, Button}}};
use crate::{SobekMsg, Views};

pub struct BlockSelectPage;

impl BlockSelectPage {
    pub fn new() -> Self {
        BlockSelectPage
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let simple = Button::new("Simple Block").on_press(SobekMsg::ChangeView(Views::Simple));
        let advanced = Button::new("Advanced Block").on_press(SobekMsg::ChangeView(Views::Advanced));

        let row = Row::new().push(simple).push(advanced).spacing(20);

        Container::new(row)
            .center_x().center_y()
            .width(Length::Fill).height(Length::Fill)
            .padding(Padding::new(200))
            .into()
    }
}
