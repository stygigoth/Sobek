use iced::{Length, pure::Element};
use iced_aw::pure::Tabs;
use crate::{SobekMsg, blockstate::BlockstateTab, model::ModelTab, loot_table::LootTab};

pub struct AdvancedBlockPage {
    pub active_tab: usize,
    pub blockstate_tab: BlockstateTab,
    pub model_tab: ModelTab,
    pub loot_tab: LootTab
}

impl AdvancedBlockPage {
    pub fn new() -> Self {
        AdvancedBlockPage {
            active_tab: 0,
            blockstate_tab: BlockstateTab::new(),
            model_tab: ModelTab::new(),
            loot_tab: LootTab::new()
        }
    }

    pub fn view(&self) -> Element<'_, SobekMsg> {
        Tabs::new(self.active_tab, SobekMsg::TabSelected)
        .push(self.blockstate_tab.tab_label(), self.blockstate_tab.view())
        .push(self.model_tab.tab_label(), self.model_tab.view())
        .push(self.loot_tab.tab_label(), self.loot_tab.view())
        .tab_bar_height(Length::Units(32))
        .tab_bar_position(iced_aw::TabBarPosition::Top)
        .into()
    }
}