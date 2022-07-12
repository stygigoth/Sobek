use iced::{Settings, pure::{Sandbox, Element}};
use std::{fs, fs::File};
use std::io::{prelude::*, Error};
use dirs;
use regex::Regex;
use crate::{main_page::MainPage, block_select::BlockSelectPage, simple_block::SimpleBlockPage};

mod main_page;
mod block_select;
mod simple_block;

static id_regex: &str = r"^[0-9a-z_.\-]+$";

fn main() -> Result<(), iced::Error> {
    let home_path = dirs::home_dir().unwrap();
    let home = home_path.display();
    let sobek = String::from("/.sobek/");
    let local_dir = format!("{}{}", home, sobek);

    match fs::create_dir_all(local_dir) {
        Err(y) => println!("could not create {}: {}", format!("{}{}", home, sobek), y),
        Ok(_) => println!("created {}", format!("{}{}", home, sobek))
    }

    Sobek::run(Settings::default())
}

fn make_folder_structure(s: &str) {
    let home_path = dirs::home_dir().unwrap();
    let home = home_path.display();
    let sobek = String::from("/.sobek/");
    let m_id = format!("{}{}", s, String::from("/"));
    let path = format!("{}{}{}", home, sobek, m_id);

    let block_model_path = format!("{}{}", path, format!("assets/{}/models/block", s));
    match fs::create_dir_all(&block_model_path) {
        Err(y) => println!("couldn't create {}: {}", &block_model_path, y),
        Ok(_) => println!("created {}", &block_model_path)
    }

    let item_model_path = format!("{}{}", path, format!("assets/{}/models/item", s));
    match fs::create_dir(&item_model_path) {
        Err(y) => println!("couldn't create {}: {}", &item_model_path, y),
        Ok(_) => println!("created {}", &item_model_path)
    }

    let blockstate_path = format!("{}{}", path, format!("assets/{}/blockstates", s));
    match fs::create_dir(&blockstate_path) {
        Err(y) => println!("couldn't create {}: {}", &blockstate_path, y),
        Ok(_) => println!("created {}", &blockstate_path)
    }

    let loot_table_path = format!("{}{}", path, format!("data/{}/loot_tables/block", s));
    match fs::create_dir_all(&loot_table_path) {
        Err(y) => println!("couldn't create {}: {}", &loot_table_path, y),
        Ok(_) => println!("created {}", &loot_table_path)
    }
}

fn create_simple_block(m: &str, s: &str, bi: &bool, ds: &bool) {
    create_simple_blockstate(m, s);
    create_simple_block_model(m, s);
    
    if *bi {
        create_simple_item_model(m, s);
        if *ds {
            create_simple_loot_table(m, s);
        }
    }
}

fn create_simple_blockstate(m: &str, s: &str) -> Result<(), Error> {
    let home_path = dirs::home_dir().unwrap();
    let home = home_path.display();
    let sobek = String::from("/.sobek/");
    let m_id = format!("{}/assets/{}/blockstates/", m, m);
    let path = format!("{}{}{}", home, sobek, m_id);

    let mut file = File::create(format!("{}{}.json", path, s))?;
    let to_write = format!("{{\n\t\"variants\": {{\n\t\t\"\": {{\n\t\t\t\"model\": \"{}:block/{}\"\n\t\t}}\n\t}}\n}}", m, s);

    file.write_all(to_write.as_bytes())?;
    Ok(())
}

fn create_simple_block_model(m: &str, s: &str) -> Result<(), Error> {
    let home_path = dirs::home_dir().unwrap();
    let home = home_path.display();
    let sobek = String::from("/.sobek/");
    let m_id = format!("{}/assets/{}/models/block/", m, m);
    let path = format!("{}{}{}", home, sobek, m_id);

    let mut file = File::create(format!("{}{}.json", path, s))?;
    let to_write = format!("{{\n\t\"parent\": \"minecraft:block/cube_all\",\n\t\"textures\": {{\n\t\t\"all\": \"{}:block/{}\"\n\t}}\n}}", m, s);

    file.write_all(to_write.as_bytes())?;
    Ok(())
}

fn create_simple_item_model(m: &str, s: &str) -> Result<(), Error> {
    let home_path = dirs::home_dir().unwrap();
    let home = home_path.display();
    let sobek = String::from("/.sobek/");
    let m_id = format!("{}/assets/{}/models/item/", m, m);
    let path = format!("{}{}{}", home, sobek, m_id);

    let mut file = File::create(format!("{}{}.json", path, s))?;
    let to_write = format!("{{\n\t\"parent\": \"{}:block/{}\"\n}}", m, s);

    file.write_all(to_write.as_bytes())?;
    Ok(())
}

fn create_simple_loot_table(m: &str, s: &str) -> Result<(), Error> {
    let home_path = dirs::home_dir().unwrap();
    let home = home_path.display();
    let sobek = String::from("/.sobek/");
    let m_id = format!("{}/data/{}/loot_tables/block/", m, m);
    let path = format!("{}{}{}", home, sobek, m_id);

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
    ConfirmSimple
}

impl Sandbox for Sobek {
    type Message = SobekMsg;

    fn new() -> Self {
        Sobek {
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
                if !Regex::new(&id_regex).unwrap().is_match(&self.main_view.mod_id) { return; }
                make_folder_structure(&self.main_view.mod_id);
                self.current_view = Views::BlockSelect
            },
            SobekMsg::ConfirmSimple => {
                if !Regex::new(&id_regex).unwrap().is_match(&self.simple_view.id) { return; }

                create_simple_block(&self.main_view.mod_id, &self.simple_view.id, &self.simple_view.has_bi, &self.simple_view.drops_self);

                self.simple_view.id = String::from("");
                self.simple_view.has_bi = false;
                self.simple_view.drops_self = false;

                self.current_view = Views::BlockSelect;
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
