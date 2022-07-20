use iced::{Length, Alignment, pure::{Element, widget::{Container, Text, TextInput, Button, Column, Row, Scrollable}}};
use iced_aw::{TabLabel, pure::Split};
use crate::{SobekMsg, block::Block};

pub struct ModelTab {
    pub split: u16,
    pub blocks: Vec<Block>,
    pub name: String,
    pub block: usize,
    default_block: Block
}

impl ModelTab {
    pub fn new () -> Self {
        ModelTab {
            split: 256,
            blocks: Vec::new(),
            name: String::from(""),
            block: 0,
            default_block: Block::new(String::from(""))
        }
    }

    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text(String::from("Model"))
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let mut col: Column<'_, SobekMsg> = Column::new().width(Length::Fill).align_items(Alignment::Center).spacing(5);
        let mut s: usize = 0;
        for x in self.blocks.iter() {
            col = col.push(Row::new().align_items(Alignment::Center).spacing(10).padding(10).push(Text::new(x.name.clone())).push(Button::new("Edit").on_press(SobekMsg::SetModelBlock(s))).push(Button::new("Delete").on_press(SobekMsg::DeleteModelBlock(s))));
            s += 1;
        }
        let scrollable = Scrollable::new(col);
        let first = Container::new(scrollable)
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill);
        let block = self.blocks.get(self.block);
        let second = if block.is_none() {Container::new(Column::new())} else {Container::new(block.unwrap_or(&self.default_block).view())};
        let second = second
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill);
        let spl = Split::new(first, second, Option::from(self.split), iced_aw::pure::split::Axis::Vertical, SobekMsg::ModelSplitSize);

        let new_btn = Button::new("+").on_press(SobekMsg::AddModelBlock(self.name.clone()));
        let new_lbl = TextInput::new("Name", &self.name, SobekMsg::ChangeModelName).padding(10);
        let new_row = Row::new().push(new_btn).push(new_lbl).align_items(Alignment::Center).spacing(5);
        let new = Container::new(new_row).center_x().center_y().width(Length::Fill);
        let import = Button::new("Import Model from Blockbench");
        let con = Container::new(import).center_x().center_y().width(Length::Fill);
        let create = Container::new(Button::new("Create Model")).center_x().center_y().width(Length::Fill);
        let row = Row::new().push(new).push(con).push(create).align_items(Alignment::Center);

        let col = Column::new().push(row).push(spl).padding(10).spacing(10);

        Container::new(col).into()
    }
}