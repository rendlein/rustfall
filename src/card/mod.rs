#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CardList {

}

#[derive(Deserialize, Debug)]
pub struct Card {
    name: String,
    mana_cost: Option<String>,
    oracle_text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    type_line: String
}

impl Card {
    pub fn new() -> Self {
        Card {
            name: "".to_string(),
            mana_cost: None,
            oracle_text: None,
            power: None,
            toughness: None,
            type_line: "".to_string()
        }
    }
}