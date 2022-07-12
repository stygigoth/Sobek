use iced::{Length, alignment::Horizontal, pure::{Element, widget::{Container, Text, Checkbox, Scrollable, Column, Row, TextInput, Button}}};
use iced_aw::{TabLabel, pure::{Modal, Card, NumberInput}};
use crate::SobekMsg;

pub struct BlockstateTab {
    pub show_modal: bool,
    pub multipart: bool,
    pub var_single: bool,
    pub variant_qual: String,
    pub model_id: String,
    pub b_id: String,
    pub x_rot: i32,
    pub y_rot: i32,
    pub uv_lock: bool,
    pub variants: Vec<(String, String)>
}

impl BlockstateTab {
    pub fn new () -> Self {
        BlockstateTab {
            show_modal: false,
            multipart: false,
            var_single: true,
            variant_qual: String::from(""),
            model_id: String::from(""),
            b_id: String::from(""),
            x_rot: 0,
            y_rot: 0,
            uv_lock: false,
            variants: Vec::new()
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
            let add_variant = Button::new("Add variant").on_press(SobekMsg::OpenAddVariant);
            col = col.push(Row::new().push(pick_type).push(add_variant).spacing(20));

            if self.var_single {
                let b_id = TextInput::new("Block ID", &self.b_id, SobekMsg::ChangeBIDA).padding(10);
                col = col.push(Row::new().push(b_id))
            } else {
                col = col.push(Row::new().push(Text::new("Not yet implemented")))
            }
        };
        let scroll = Scrollable::new(col);

        let content: Element<'_, SobekMsg> = Container::new(scroll)
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill)
        .padding(200).into();

        Modal::new(self.show_modal, content, || {
            let header = Row::new().push(Text::new("Add variant")).padding(10);
            let qualifier = Row::new().push(TextInput::new("Variant qualifier", &self.variant_qual, SobekMsg::VariantQual).padding(5)).padding(10);
            let model = Row::new().push(TextInput::new("Model ID", &self.model_id, SobekMsg::BlockstateModel).padding(5)).padding(10);
            let rotation_labels = Row::new().push(Text::new("      X Rotation")).push(Text::new("Y Rotation")).push(Checkbox::new(self.uv_lock, "UV Lock?", SobekMsg::BlockstateUV)).spacing(65);
            let rotations = Row::new().push(NumberInput::new(self.x_rot, i32::from(360), SobekMsg::BlockstateXrotChange).step(1).min(0)).push(NumberInput::new(self.y_rot, i32::from(360), SobekMsg::BlockstateYrotChange).step(1).min(0)).padding(10).spacing(10);
            let col1 = Column::new().push(qualifier).push(model).push(rotation_labels).push(rotations).spacing(5);
            Card::new(
                header,
                col1
            ).foot(
                Row::new().spacing(10).padding(5).width(Length::Fill)
                    .push(
                        Button::new(Text::new("Cancel").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill).on_press(SobekMsg::CloseAddVariant)
                    )
                    .push(
                        Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill).on_press(SobekMsg::SubmitAddVariant)   
                    )
            ).max_width(500).into()
        }).backdrop(SobekMsg::CloseAddVariant).on_esc(SobekMsg::CloseAddVariant).into()
    }
}