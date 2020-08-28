use crate::card::CardList;
use reqwest::blocking as reqwest;

#[derive(Debug)]
pub struct Query {
    pub string: String,
}

impl Query {
    pub fn new(mut req: String) -> Self {
        let mut search = "https://api.scryfall.com/cards/search?q=".to_string();
        if req.contains("scryfall.com") {
            req = req.replace("https://api.scryfall.com/cards/search", "");
        }
        search.push_str(format(&req).as_str());
        Query { string: search }
    }

    /// Returns the String for the Next Page
    pub fn run(self) -> Option<CardList> {
        let response = reqwest::get(self.string.as_str());
        match response {
            Ok(response) => {
                let results = response.json::<Option<CardList>>();
                match results {
                    Ok(r) => r,
                    Err(_e) => std::process::exit(1),
                }
            }
            Err(_e) => {
                std::process::exit(1);
            }
        }
    }
}

fn format(str: &str) -> String {
    str.replace(":", "%3A")
        //.replace("=", "%3D")
        .replace(" ", "+")
}
