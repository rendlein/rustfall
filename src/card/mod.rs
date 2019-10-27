#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Card {
    name: String,
    type_line: String,
    mana_cost: String
}