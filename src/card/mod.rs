use serde::Deserialize;
use std::collections::HashMap;
use std::{thread, time};
use crate::query::Query;

#[derive(Deserialize, Debug)]
pub struct CardList {
    object: String,
    total_cards: u32,
    has_more: bool,
    next_page: Option<String>,
    pub data: Vec<Card>
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

    pub fn has_more(&self) -> bool {
        self.has_more
    }

    /// Return number of pages for the query.
    /// Scryfall shows 175 cards per page.
    pub fn pages(&self) -> u32 {
        (self.total_cards / 175) + 1
    }

    #[allow(unused_assignments)]
    pub fn process(&mut self) {
        if self.has_more {
            let mut list: Option<CardList> = None;
            let mut next = self.next_page.as_ref().unwrap().clone();
            let mut count: u32 = 2;
            let pages = self.pages();
            while count <= pages {
                if pages > 5 {
                    let delay = time::Duration::from_millis(50);
                    thread::sleep(delay);
                }
                let query = Query::new(next.clone());
                list = query.run();

                match list {
                    Some(item) => {
                        for card in item.data {
                            self.data.push(card);

                            if item.has_more {
                                next = item.next_page.as_ref().unwrap().clone();
                            }
                        }
                    }
                    _ => {}
                }

                count += 1;
            }
        }
    }
}

impl Card {
    pub fn print(&self) {
        match &self.card_faces {
            Some(faces) => {
                for card in faces {
                    card.print();
                    println!();
                }
            }
            _ => {
                self._print();
                println!();
            }
        }
    }

    fn _print(&self) {
        println!("{}", self.name);

        match &self.mana_cost {
            Some(cost) => {
                if cost != "" {
                    println!("{}", cost)
                }
            }
            _ => ()
        };

        println!("{}", self.type_line);

        match &self.oracle_text {
            Some(oracle) => {
                if oracle != "" {
                    println!("{}", oracle)
                }
            }
            _ => {}
        };

        match &self.power {
            Some(pow) => { print!("{}/", pow) }
            _ => {}
        };

        match &self.toughness {
            Some(tough) => { println!("{}", tough) }
            _ => {}
        };

        match &self.loyalty {
            Some(loyal) => { println!("{}", loyal) }
            _ => {}
        };
    }
}

impl CardFace {
    pub fn print(&self) {
        println!("{}", self.name);
        match &self.mana_cost {
            Some(cost) => {
                if *cost != "".to_string() {
                    println!("{}", cost);
                }
            }
            None => {}
        }

        println!("{}", self.type_line);

        match &self.oracle_text {
            Some(oracle) => {
                println!("{}", oracle);
            }
            None => {}
        };

        match &self.power {
            Some(power) => { print!("{}", power) },
            None => {},
        };

        match &self.toughness {
            Some(tough) => { println!("/{}", tough) },
            None => {},
        };

        match &self.loyalty {
            Some(loyal) => { println!("Loyalty: {}", loyal) },
            None => {},
        };

    }
}