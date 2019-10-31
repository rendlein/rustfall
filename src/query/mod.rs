use reqwest;
use crate::card::CardList;

#[derive(Debug)]
pub struct Query {
    pub string: String
}

impl Query {
    pub fn new(mut req: String) -> Self {
        let mut search = "https://api.scryfall.com/cards/search?q=".to_string();
        if req.contains("scryfall.com") {
            req = req.replace("https://api.scryfall.com/cards/search", "");
        }
        search.push_str(format(req).as_str());
        Query {
            string: search
        }
    }

    /// Returns the String for the Next Page
    pub fn run(self) -> Option<CardList> {
        let mut res: Option<CardList> = None;
        let response = reqwest::get(self.string.as_str());
        if response.is_ok() {
            let mut results = response.unwrap();
            if results.status().is_client_error() || results.status().is_server_error() {
                std::process::exit(1);
            }
            res = results.json().unwrap();
        }
        res
    }
}

fn format(str: String) -> String {
    str.replace(":", "%3A")
        //.replace("=", "%3D")
        .replace(" ", "+")
}
