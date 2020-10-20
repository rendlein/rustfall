use crate::card::CardList;
use ::reqwest::blocking as reqwest;

#[derive(Debug)]
pub struct Query {
    pub string: String,
}

impl Query {
    pub fn new(req: String) -> Self {
        let mut search = "https://api.scryfall.com/cards/search?q=".to_string();

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
                    Ok(r) =>  {
                        r
                    },
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

#[cfg(test)]
mod tests {
    use crate::query::Query;

    #[test]
    fn build_api_query() {
        let q = Query::new(String::from("f:pauper t:creature id:gb"));
        assert_eq!("https://api.scryfall.com/cards/search?q=f%3Apauper+t%3Acreature+id%3Agb", q.string);
    }
}