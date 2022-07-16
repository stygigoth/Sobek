use iced::pure::Element;
use iced_aw::TabLabel;
use crate::{SobekMsg, blockstate_variant::BlockstateVariant, blockstate_multipart::BlockstateMultipart};

#[derive(Debug, Clone)]
pub enum BlockstateViews {
    Variant,
    Multipart
}

pub struct BlockstateTab {
    pub view: BlockstateViews,
    pub variant_view: BlockstateVariant,
    pub multipart_view: BlockstateMultipart,
    pub multipart: bool
}

impl BlockstateTab {
    pub fn new () -> Self {
        BlockstateTab {
            view: BlockstateViews::Variant,
            variant_view: BlockstateVariant::new(),
            multipart_view: BlockstateMultipart::new(),
            multipart: false
        }
    }

    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text(String::from("Blockstate"))
    }

    pub fn view(&self) -> Element<SobekMsg> {
        match self.view {
            BlockstateViews::Variant => self.variant_view.view(self.multipart),
            BlockstateViews::Multipart => self.multipart_view.view(self.multipart)
        }
    }
}