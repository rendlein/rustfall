#![allow(dead_code)]
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct CardList {
    object: String,
    total_cards: u32,
    has_more: bool,
    next_page: Option<String>,
    data: Vec<Card>
}

#[derive(Deserialize, Debug)]
pub struct Card {
    object: String,
    id: String,
    oracle_id: String,
    multiverse_ids: Vec<u32>,
    name: String,
    printed_name: Option<String>,
    lang: String,
    released_at: String,
    uri: String,
    scryfall_uri: String,
    layout: String,
    highres_image: bool,
    image_uris: Option<HashMap<String, Option<String>>>,
    mana_cost: Option<String>,
    cmc: f32,
    type_line: String,
    oracle_text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
    colors: Option<Vec<String>>,
    color_indicator: Option<Vec<String>>,
    color_identity: Vec<String>,
    card_faces: Option<Vec<CardFace>>,
    legalities: HashMap<String, String>,
    games: Vec<String>,
    reserved: bool,
    foil: bool,
    nonfoil: bool,
    oversized: bool,
    promo: bool,
    reprint: bool,
    variation: bool,
    set: String,
    set_name: String,
    set_type: String,
    set_uri: String,
    set_search_uri: String,
    scryfall_set_uri: String,
    rulings_uri: String,
    prints_search_uri: String,
    collector_number: String,
    digital: bool,
    rarity: String,
    card_back_id: String,
    artist: String,
    artist_ids: Vec<String>,
    illustration_id: Option<String>,
    border_color: String,
    frame: String,
    full_art: bool,
    textless: bool,
    booster: bool,
    story_spotlight: bool,
    prices: HashMap<String, Option<String>>,
    related_uris: HashMap<String, String>,
    purchase_uris: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct CardFace {
    object: String,
    name: String,
    mana_cost: Option<String>,
    type_line: String,
    oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_indicator: Option<Vec<String>>,
    loyalty: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    artist: Option<String>,
    artist_id: Option<String>,
    illustration_id: Option<String>,
    image_uris: Option<HashMap<String, String>>,

    flavor_text: Option<String>,
    printed_name: Option<String>,
    printed_text: Option<String>,
    printed_type_line: Option<String>,
    watermark: Option<String>
}

impl CardList {
    pub fn print_list(&self) {
        let cards = self.data.iter();

        for item in cards {
            item.print();
        }
    }
}

impl Card {
    pub fn print(&self) {
        match &self.card_faces {
            Some(x) => {
                for i in x {
                    i.print();
                }
            }
            _ => ()
        }
        self._print();
    }

    fn _print(&self) {
        println!("{}", self.name);

        match &self.mana_cost {
            Some(x) => { println!("{}", x) }
            _ => ()
        };

        println!("{}", self.type_line);

        match &self.oracle_text {
            Some(x) => { println!("{}", x) }
            _ => ()
        };

        match &self.power {
            Some(pow) => { print!("{}/", pow) }
            _ => ()
        };

        match &self.toughness {
            Some(tough) => { println!("{}", tough) }
            _ => ()
        };

        match &self.loyalty {
            Some(loyal) => { println!("{}", loyal) }
            _ => ()
        };
    }
}

impl CardFace {
    pub fn print(&self) {
        println!("{}", self.name);
        match &self.mana_cost {
            Some(x) => {
                if *x != "".to_string() {
                    println!("{}", x);
                }
            }
            _ => ()
        };
    }
}