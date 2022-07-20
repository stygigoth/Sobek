use iced::{Length, Alignment, pure::{widget::{Column, Row, Container, Text}, Element}};
use crate::SobekMsg;

pub struct Block {
    pub name: String,
    offset: (u32, u32, u32),
    size: (f64, f64, f64),
    pivot: (u32, u32, u32),
    rotation: (char, i16)
}

impl Block {
    pub fn new(n: String) -> Self {
        Block {
            name: n,
            offset: (0, 0, 0),
            size: (1.0, 1.0, 1.0),
            pivot: (0, 0, 0),
            rotation: ('x', 0)
        }
    }

    pub fn get_offset(&self) -> (u32, u32, u32) {
        self.offset.clone()
    }

    pub fn get_size(&self) -> (f64, f64, f64) {
        self.size.clone()
    }

    pub fn get_pivot(&self) -> (u32, u32, u32) {
        self.get_pivot().clone()
    }

    pub fn get_rotation(&self) -> (char, i16) {
        self.rotation.clone()
    }

    pub fn set_offset(&mut self, offset: (u32, u32, u32)) {
        self.offset = offset;
    }

    pub fn set_size(&mut self, size: (f64, f64, f64)) {
        self.size = size;
    }

    pub fn set_pivot(&mut self, pivot: (u32, u32, u32)) {
        self.pivot = pivot;
    }

    pub fn set_rotation(&mut self, c: char, r: i16) {
        if c != 'x' && c != 'y' && c != 'z' { panic!("invalid rotation direction: {}! must be one of: ['x', 'y', 'z']", c); }

        self.rotation = (c, r);
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let name: Row<'_, SobekMsg> = Row::new().push(Text::new(self.name.clone()));
        let col = Column::new().spacing(20).padding(10).align_items(Alignment::Center)
        .push(name);

        Container::new(col).center_x().center_y().width(Length::Fill).height(Length::Fill).into()
    }
}