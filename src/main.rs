use iced::{Settings, pure::{Sandbox, Element}};
use std::{fs, fs::File};
use std::io::{prelude::*, Error};
use regex::Regex;
use dirs;
use native_dialog::FileDialog;
use crate::{main_page::MainPage, block_select::BlockSelectPage, simple_block::SimpleBlockPage, advanced_block::AdvancedBlockPage};

mod main_page;
mod block_select;
mod simple_block;
mod advanced_block;
mod blockstate;
mod model;
mod loot_table;
mod create;

static ID_REGEX: &str = r"^[0-9a-z_.\-]+$";

fn main() -> Result<(), iced::Error> {
    Sobek::run(Settings::default())
}

fn make_folder_structure(d: &str, s: &str) {
    let path = format!("{}/", d);

    let block_model_path = format!("{}{}", path, format!("assets/{}/models/block", s));
    match fs::create_dir_all(&block_model_path) {
        Err(y) => println!("couldn't create {}: {}", &block_model_path, y),
        Ok(_) => println!("created {}", &block_model_path)
    }

    let item_model_path = format!("{}{}", path, format!("assets/{}/models/item", s));
    match fs::create_dir_all(&item_model_path) {
        Err(y) => println!("couldn't create {}: {}", &item_model_path, y),
        Ok(_) => println!("created {}", &item_model_path)
    }

    let blockstate_path = format!("{}{}", path, format!("assets/{}/blockstates", s));
    match fs::create_dir_all(&blockstate_path) {
        Err(y) => println!("couldn't create {}: {}", &blockstate_path, y),
        Ok(_) => println!("created {}", &blockstate_path)
    }

    let loot_table_path = format!("{}{}", path, format!("data/{}/loot_tables/block", s));
    match fs::create_dir_all(&loot_table_path) {
        Err(y) => println!("couldn't create {}: {}", &loot_table_path, y),
        Ok(_) => println!("created {}", &loot_table_path)
    }
}

fn create_simple_block(d: &str, m: &str, s: &str, bi: &bool, ds: &bool) {
    match create_simple_blockstate(d, m, s) {
        Err(y) => println!("couldn't create blockstate: {}", y),
        Ok(_) => println!("created blockstate for: {}:{}", m, s)
    }
    match create_simple_block_model(d, m, s) {
        Err(y) => println!("couldn't create block model: {}", y),
        Ok(_) => println!("created block model for: {}:{}", m, s)
    }
    
    if *bi {
        match create_simple_item_model(d, m, s) {
            Err(y) => println!("couldn't create item model: {}", y),
            Ok(_) => println!("created item model for: {}:{}", m, s)
        }

        if *ds {
            match create_simple_loot_table(d, m, s) {
                Err(y) => println!("couldn't create loot table: {}", y),
                Ok(_) => println!("created loot table for: {}:{}", m, s)
            }
        }
    }
}

fn create_simple_blockstate(d: &str, m: &str, s: &str) -> Result<(), Error> {
    let m_id = format!("assets/{}/blockstates/", m);
    let path = format!("{}{}", d, m_id);

    let mut file = File::create(format!("{}{}.json", path, s))?;
    let to_write = format!("{{\n\t\"variants\": {{\n\t\t\"\": {{\n\t\t\t\"model\": \"{}:block/{}\"\n\t\t}}\n\t}}\n}}", m, s);

    file.write_all(to_write.as_bytes())?;
    Ok(())
}

fn create_simple_block_model(d: &str, m: &str, s: &str) -> Result<(), Error> {
    let m_id = format!("assets/{}/models/block/", m);
    let path = format!("{}{}", d, m_id);

    let mut file = File::create(format!("{}{}.json", path, s))?;
    let to_write = format!("{{\n\t\"parent\": \"minecraft:block/cube_all\",\n\t\"textures\": {{\n\t\t\"all\": \"{}:block/{}\"\n\t}}\n}}", m, s);

    file.write_all(to_write.as_bytes())?;
    Ok(())
}

fn create_simple_item_model(d: &str, m: &str, s: &str) -> Result<(), Error> {
    let m_id = format!("assets/{}/models/item/", m);
    let path = format!("{}{}", d, m_id);

    let mut file = File::create(format!("{}{}.json", path, s))?;
    let to_write = format!("{{\n\t\"parent\": \"{}:block/{}\"\n}}", m, s);

    file.write_all(to_write.as_bytes())?;
    Ok(())
}

fn create_simple_loot_table(d: &str, m: &str, s: &str) -> Result<(), Error> {
    let m_id = format!("data/{}/loot_tables/block/", m);
    let path = format!("{}{}", d, m_id);

    let mut file = File::create(format!("{}{}.json", path, s))?;
    let to_write = format!("{{\n\t\"type\": \"minecraft:block\",\n\t\"pools\": [\n\t\t{{\n\t\t\t\"bonus_rolls\": 0.0,\n\t\t\t\"conditions\": [\n\t\t\t\t{{\n\t\t\t\t\t\"condition\": \"minecraft:survives_explosion\"\n\t\t\t\t}}\n\t\t\t],\n\t\t\t\"entries\": [\n\t\t\t\t{{\n\t\t\t\t\t\"type\": \"minecraft:item\",\n\t\t\t\t\t\"name\": \"{}:{}\"\n\t\t\t\t}}\n\t\t\t],\n\t\t\t\"rolls\": 1.0\n\t\t}}\n\t]\n}}", m, s);

    file.write_all(to_write.as_bytes())?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum Views {
    Main,
    BlockSelect,
    Simple,
    Advanced
}

struct Sobek {
    working_directory: String,
    current_view: Views,
    main_view: MainPage,
    block_select_view: BlockSelectPage,
    simple_view: SimpleBlockPage,
    advanced_view: AdvancedBlockPage
}

#[derive(Debug, Clone)]
pub enum SobekMsg {
    ChangeMID(String),
    ChangeView(Views),
    ConfirmMID,
    ChangeBID(String),
    ChangeBIDA(String),
    ToggleBI(bool),
    ToggleDS(bool),
    ConfirmSimple,
    SelectDir,
    TabSelected(usize),
    LootSplitSize(u16),
    ModelSplitSize(u16),
    BlockstateTypeChange(bool),
    VarChange(bool),
    Create,
    OpenAddVariant,
    CloseAddVariant,
    SubmitAddVariant,
    VariantQual(String),
    BlockstateModel(String),
    BlockstateXrotChange(i32),
    BlockstateYrotChange(i32),
    BlockstateUV(bool)
}

impl Sandbox for Sobek {
    type Message = SobekMsg;

    fn new() -> Self {
        Sobek {
            working_directory: String::from(""),
            current_view: Views::Advanced,
            main_view: MainPage::new(),
            block_select_view: BlockSelectPage::new(),
            simple_view: SimpleBlockPage::new(),
            advanced_view: AdvancedBlockPage::new()
        }
    }

    fn title(&self) -> String {
        match self.current_view {
            Views::Main => String::from("Sobek"),
            Views::BlockSelect => String::from("Sobek"),
            Views::Advanced => String::from("Sobek - Advanced Block"),
            Views::Simple => String::from("Sobek - Simple Block")
        }
    }

    fn update(&mut self, message: Self::Message) {
        let mut a: i8 = 0;
        match message {
            SobekMsg::ChangeMID(s) => self.main_view.mod_id = s,
            SobekMsg::ChangeBID(s) => self.simple_view.id = s,
            SobekMsg::ChangeBIDA(s) => self.advanced_view.blockstate_tab.b_id = s,
            SobekMsg::ToggleBI(b) => self.simple_view.has_bi = b,
            SobekMsg::ToggleDS(b) => self.simple_view.drops_self = b,
            SobekMsg::ChangeView(v) => self.current_view = v,
            SobekMsg::ConfirmMID => {
                if !Regex::new(&ID_REGEX).unwrap().is_match(&self.main_view.mod_id) || self.working_directory == "" { return; }
                make_folder_structure(&self.working_directory, &self.main_view.mod_id);
                self.current_view = Views::BlockSelect
            },
            SobekMsg::ConfirmSimple => {
                if !Regex::new(&ID_REGEX).unwrap().is_match(&self.simple_view.id) { return; }

                create_simple_block(&self.working_directory, &self.main_view.mod_id, &self.simple_view.id, &self.simple_view.has_bi, &self.simple_view.drops_self);

                self.simple_view.id = String::from("");
                self.simple_view.has_bi = false;
                self.simple_view.drops_self = false;

                self.current_view = Views::BlockSelect;
            },
            SobekMsg::SelectDir => {
                self.working_directory = String::from(FileDialog::new()
                    .set_location(dirs::home_dir().unwrap().as_path())
                    .add_filter("Directory", &[""])
                    .show_open_single_dir()
                    .unwrap().unwrap().as_os_str().to_str().unwrap());
            },
            SobekMsg::TabSelected(sel) => {
                if sel != 3 { self.advanced_view.active_tab = sel; }
                else { drop(SobekMsg::Create); }
            },
            SobekMsg::LootSplitSize(size) => self.advanced_view.loot_tab.split = size,
            SobekMsg::ModelSplitSize(size) => self.advanced_view.model_tab.split = size,
            SobekMsg::BlockstateTypeChange(type_of) => self.advanced_view.blockstate_tab.multipart = type_of,
            SobekMsg::VarChange(b) => self.advanced_view.blockstate_tab.var_single = b,
            SobekMsg::Create => a = 1,
            SobekMsg::OpenAddVariant => self.advanced_view.blockstate_tab.show_modal = true,
            SobekMsg::CloseAddVariant => self.advanced_view.blockstate_tab.show_modal = false,
            SobekMsg::SubmitAddVariant => {
                if self.advanced_view.blockstate_tab.model_id != "" {
                    todo!();
                }

                self.advanced_view.blockstate_tab.show_modal = false
            }
            SobekMsg::VariantQual(s) => self.advanced_view.blockstate_tab.variant_qual = s,
            SobekMsg::BlockstateModel(s) => self.advanced_view.blockstate_tab.model_id = s,
            SobekMsg::BlockstateXrotChange(i) => self.advanced_view.blockstate_tab.x_rot = i,
            SobekMsg::BlockstateYrotChange(i) => self.advanced_view.blockstate_tab.y_rot = i,
            SobekMsg::BlockstateUV(b) => self.advanced_view.blockstate_tab.uv_lock = b,
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self.current_view {
            Views::Main => self.main_view.view(),
            Views::BlockSelect => self.block_select_view.view(),
            Views::Simple => self.simple_view.view(),
            Views::Advanced => self.advanced_view.view()
        }
    }
}
