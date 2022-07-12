use iced::{Settings, Error, pure::{Sandbox, Element}};
use std::fs;
use dirs;
use crate::{main_page::MainPage, block_select::BlockSelectPage};

mod main_page;
mod block_select;

fn main() -> Result<(), Error> {
    let home_path = dirs::home_dir().unwrap();
    let home = home_path.display();
    let sobek = String::from("/.sobek/");
    let local_dir = format!("{}{}", home, sobek);

    match fs::create_dir_all(local_dir) {
        Err(y) => panic!("could not create {}: {}", format!("{}{}", home, sobek), y),
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
        Err(y) => panic!("couldn't create {}: {}", &block_model_path, y),
        Ok(_) => println!("created {}", &block_model_path)
    }

    let item_model_path = format!("{}{}", path, format!("assets/{}/models/item", s));
    match fs::create_dir(&item_model_path) {
        Err(y) => panic!("couldn't create {}: {}", &item_model_path, y),
        Ok(_) => println!("created {}", &item_model_path)
    }

    let blockstate_path = format!("{}{}", path, format!("assets/{}/blockstates", s));
    match fs::create_dir(&blockstate_path) {
        Err(y) => panic!("couldn't create {}: {}", &blockstate_path, y),
        Ok(_) => println!("created {}", &blockstate_path)
    }

    let loot_table_path = format!("{}{}", path, format!("data/{}/loot_tables/block", s));
    match fs::create_dir_all(&loot_table_path) {
        Err(y) => panic!("couldn't create {}: {}", &loot_table_path, y),
        Ok(_) => println!("created {}", &loot_table_path)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Views {
    Main,
    BlockSelect
}

struct Sobek {
    current_view: Views,
    main_view: MainPage,
    block_select_view: BlockSelectPage
}

#[derive(Debug, Clone)]
pub enum SobekMsg {
    ChangeMID(String),
    ChangeView(Views),
    ConfirmMID
}

impl Sandbox for Sobek {
    type Message = SobekMsg;

    fn new() -> Self {
        Sobek {
            current_view: Views::Main,
            main_view: MainPage::new(),
            block_select_view: BlockSelectPage::new()
        }
    }

    fn title(&self) -> String {
        String::from("Sobek")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            SobekMsg::ChangeMID(s) => self.main_view.mod_id = s,
            SobekMsg::ChangeView(v) => self.current_view = v,
            SobekMsg::ConfirmMID => {
                make_folder_structure(&self.main_view.mod_id);

                self.current_view = if self.main_view.mod_id != "" {Views::BlockSelect} else {self.current_view}
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self.current_view {
            Views::Main => self.main_view.view(),
            Views::BlockSelect => self.block_select_view.view()
        }
    }
}
