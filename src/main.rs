use iced::{Settings, pure::{Sandbox, Element}};
use std::{fs, fs::File};
use std::io::{prelude::*, Error};
use regex::Regex;
use dirs;
use native_dialog::FileDialog;
use crate::{main_page::MainPage, block_select::BlockSelectPage, simple_block::SimpleBlockPage};

mod main_page;
mod block_select;
mod simple_block;

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
    create_simple_blockstate(d, m, s);
    create_simple_block_model(d, m, s);
    
    if *bi {
        create_simple_item_model(d, m, s);
        if *ds {
            create_simple_loot_table(d, m, s);
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
    Simple
}

struct Sobek {
    working_directory: String,
    current_view: Views,
    main_view: MainPage,
    block_select_view: BlockSelectPage,
    simple_view: SimpleBlockPage
}

#[derive(Debug, Clone)]
pub enum SobekMsg {
    ChangeMID(String),
    ChangeView(Views),
    ConfirmMID,
    ChangeBID(String),
    ToggleBI(bool),
    ToggleDS(bool),
    ConfirmSimple,
    SelectDir
}

impl Sandbox for Sobek {
    type Message = SobekMsg;

    fn new() -> Self {
        Sobek {
            working_directory: String::from(""),
            current_view: Views::Main,
            main_view: MainPage::new(),
            block_select_view: BlockSelectPage::new(),
            simple_view: SimpleBlockPage::new()
        }
    }

    fn title(&self) -> String {
        String::from("Sobek")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            SobekMsg::ChangeMID(s) => self.main_view.mod_id = s,
            SobekMsg::ChangeBID(s) => self.simple_view.id = s,
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
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self.current_view {
            Views::Main => self.main_view.view(),
            Views::BlockSelect => self.block_select_view.view(),
            Views::Simple => self.simple_view.view()
        }
    }
}
