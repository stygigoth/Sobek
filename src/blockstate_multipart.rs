use iced::{Length, alignment::Horizontal, Alignment, pure::{Element, widget::{Container, Text, Checkbox, Scrollable, Column, Row, TextInput, Button}}};
use iced_aw::pure::{Modal, Card, NumberInput};
use crate::SobekMsg;

pub struct BlockstateMultipart {
    pub show_modal: bool,
    pub show_part_modal: bool,
    pub show_when_modal: bool,
    pub model_id: String,
    pub x_rot: i32,
    pub y_rot: i32,
    pub weight: i64,
    pub uv_lock: bool,
    pub name: String,
    pub ifw: bool,
    pub when: Vec<(String, bool)>,
    pub variants: Vec<(String, i32, i32, i64, bool)>,
    pub parts: Vec<(Vec<(String, bool)>, Vec<(String, i32, i32, i64, bool)>)>
}

impl BlockstateMultipart {
    pub fn new () -> Self {
        BlockstateMultipart {
            show_modal: false,
            show_part_modal: false,
            show_when_modal: false,
            model_id: String::from(""),
            x_rot: 0,
            y_rot: 0,
            weight: 1,
            uv_lock: false,
            name: String::from(""),
            ifw: false,
            when: Vec::new(),
            variants: Vec::new(),
            parts: Vec::new(),
        }
    }

    pub fn view(&self, multipart: bool) -> Element<SobekMsg> {
        let pick_type = Checkbox::new(multipart, "Multipart?", SobekMsg::BlockstateTypeChange);
        let mut col: Column<'_, SobekMsg> = Column::new().padding(10).spacing(10);
        let add_part = Button::new("Add part").on_press(SobekMsg::OpenAddPart);
        col = col.push(Row::new().push(pick_type).push(add_part).spacing(20).align_items(Alignment::Center));
        
        for x in self.parts.iter() {
            col = col.push(Row::new().push(Text::new("________________________________").horizontal_alignment(Horizontal::Center)));

            for y in x.1.iter() {
                col = col.push(Row::new().push(Text::new(format!("Model: {}", y.0)).horizontal_alignment(Horizontal::Center)).push(Text::new(format!("Weight: {}", y.3)).horizontal_alignment(Horizontal::Center)).push(Text::new(format!("UV Lock: {}", y.4)).horizontal_alignment(Horizontal::Center)).spacing(20));
                col = col.push(Row::new().push(Text::new(format!("X Rotation: {}", y.1)).horizontal_alignment(Horizontal::Center)).push(Text::new(format!("Y Rotation: {}", y.2)).horizontal_alignment(Horizontal::Center)).spacing(20));
            }
            col = col.push(Row::new().push(Text::new("-----------------------------------").horizontal_alignment(Horizontal::Center)));
            for y in x.0.iter() {
                col = col.push(Row::new().push(Text::new(format!("{}: {}", y.0, y.1))));
            }
        }

        let scroll = Scrollable::new(col);

        let content: Element<'_, SobekMsg> = Container::new(scroll)
        .center_x().center_y()
        .width(Length::Fill).height(Length::Fill)
        .padding(100).into();

        let inner: Element<'_, SobekMsg> = Modal::new(self.show_part_modal, content, || {
            let header = Row::new().push(Text::new("Add part")).padding(10);
            let add_variant = Button::new("Add variant").on_press(SobekMsg::MPOpenAddVariant);
            let clear_variants = Button::new("Clear variants").on_press(SobekMsg::MPClearVariants);
            let add_condition = Button::new("Add condition").on_press(SobekMsg::AddCondition);
            let clear_conditions = Button::new("Clear conditions").on_press(SobekMsg::ClearConditions);
            let buttons = Row::new().push(add_variant).push(clear_variants).push(add_condition).push(clear_conditions).align_items(Alignment::Center).width(Length::Fill).spacing(20).padding(10);

            let mut col1: Column<'_, SobekMsg> = Column::new().push(Text::new("Variants:")).spacing(10);
            for x in self.variants.iter() {
                col1 = col1.push(
                    Row::new().push(Text::new(format!("Model: {}", x.0))).push(Text::new(format!("UV Lock: {}", x.4))).spacing(20)
                ).push(
                    Row::new().push(Text::new(format!("X Rotation: {}", x.1))).push(Text::new(format!("Y Rotation: {}", x.2))).push(Text::new(format!("Weight: {}", x.3))).spacing(20)
                )
            }
            let mut col2: Column<'_, SobekMsg> = Column::new().push(Text::new("Conditions:")).spacing(10);
            for x in self.when.iter() {
                col2 = col2.push(
                    Text::new(format!("{}: {}", x.0, x.1))
                )
            }
            
            let scrollable = Scrollable::new(col1);
            let scrollable_1 = Scrollable::new(col2);
            let row = Row::new().push(scrollable).push(scrollable_1).spacing(20).align_items(Alignment::Fill).height(Length::Units(180));

            let body = Column::new().push(buttons).push(row).align_items(Alignment::Center).spacing(10);
            Card::new(header, body)
            .foot(
                Row::new().spacing(10).padding(5).width(Length::Fill)
                    .push(
                        Button::new(Text::new("Cancel").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill).on_press(SobekMsg::CloseAddPart)
                    )
                    .push(
                        Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill).on_press(SobekMsg::SubmitAddPart)
                    )
            ).max_height(750).max_width(750).into()
        }).backdrop(SobekMsg::CloseAddPart).on_esc(SobekMsg::CloseAddPart).into();

        let outer: Element<'_, SobekMsg> = Modal::new(self.show_modal, inner, || {
            let header = Row::new().push(Text::new("Add variant")).padding(10);
            let model = Row::new().push(TextInput::new("Model ID", &self.model_id, SobekMsg::MPBlockstateModel).padding(5)).padding(10);
            let rotations = Row::new()
            .push(Column::new().push(Text::new("X Rotation")).push(NumberInput::new(self.x_rot, i32::from(360), SobekMsg::MPBlockstateXrotChange).step(90).min(0)).padding(10).spacing(10).align_items(Alignment::Center))
            .push(Column::new().push(Text::new("Y Rotation")).push(NumberInput::new(self.y_rot, i32::from(360), SobekMsg::MPBlockstateYrotChange).step(90).min(0)).padding(10).spacing(10).align_items(Alignment::Center))
            .push(Column::new().push(Text::new("Weight")).push(NumberInput::new(self.weight, 10000000000000000, SobekMsg::MPBlockstateWeightChange).step(1).min(1)).padding(10).spacing(10).align_items(Alignment::Center))
            .push(Checkbox::new(self.uv_lock, "UV Lock?", SobekMsg::MPBlockstateUV)).align_items(Alignment::Center).padding(10).spacing(15);
            let col1 = Column::new().push(model).push(rotations).spacing(5);
            Card::new(
                header,
                col1
            ).foot(
                Row::new().spacing(10).padding(5).width(Length::Fill)
                    .push(
                        Button::new(Text::new("Cancel").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill).on_press(SobekMsg::MPCloseAddVariant)
                    )
                    .push(
                        Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill).on_press(SobekMsg::MPSubmitAddVariant)   
                    )
            ).max_width(750).into()
        }).backdrop(SobekMsg::MPCloseAddVariant).on_esc(SobekMsg::MPCloseAddVariant).into();

        Modal::new(self.show_when_modal, outer, || {
            let header = Row::new().push(Text::new("Add condition")).padding(10);

            let name = TextInput::new("Condition name", &self.name, SobekMsg::MPWhenName).padding(10);
            let tf = Checkbox::new(self.ifw, "True", SobekMsg::ConditionChange);
            let body = Row::new().push(name).push(tf).spacing(20).align_items(Alignment::Center);

            Card::new(header, body)
            .foot(Row::new().spacing(10).padding(5).width(Length::Fill)
            .push(
                Button::new(Text::new("Cancel").horizontal_alignment(Horizontal::Center))
                    .width(Length::Fill).on_press(SobekMsg::MPCloseAddVariant)
            )
            .push(
                Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                    .width(Length::Fill).on_press(SobekMsg::MPSubmitAddCondition)
            )).max_width(750).into()
        }).backdrop(SobekMsg::MPCloseAddVariant).on_esc(SobekMsg::MPCloseAddVariant).into()
    }
}