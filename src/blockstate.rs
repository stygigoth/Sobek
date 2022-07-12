use iced::{Length, pure::{Element, widget::{Container, Text, Checkbox, Scrollable, Column, Row, TextInput, Button}}};
use iced_aw::TabLabel;
use crate::SobekMsg;

pub struct BlockstateTab {
    pub multipart: bool,
    pub var_single: bool,
    pub b_id: String
}

impl BlockstateTab {
    pub fn new () -> Self {
        BlockstateTab {
            multipart: false,
            var_single: true,
            b_id: String::from("")
        }
    }

    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text(String::from("Blockstate"))
    }

    pub fn view(&self) -> Element<SobekMsg> {
        let pick_type = Checkbox::new(self.multipart, "Multipart?" , SobekMsg::BlockstateTypeChange);
        let mut col: Column<'_, SobekMsg> = Column::new().padding(10).spacing(10);
        if self.multipart {
            SobekMsg::VarChange(false);
            col = col.push(Row::new().push(pick_type).push(Text::new(String::from("Not yet implemented"))).spacing(20))
        } else {
            let var_single = Checkbox::new(self.var_single, "Single?", SobekMsg::VarChange);
            col = col.push(Row::new().push(pick_type).push(var_single).spacing(20));

            if self.var_single {
                let b_id = TextInput::new("Block ID", &self.b_id, SobekMsg::ChangeBIDA).padding(10);
                let create = Button::new("Create Blockstate");
                col = col.push(Row::new().push(b_id).push(create).spacing(20))
            } else {
                col = col.push(Row::new().push(Text::new("Not yet implemented")))
            }
        };
        let scroll = Scrollable::new(col);

        Container::new(scroll)
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill)
        .padding(200).into()
    }
}