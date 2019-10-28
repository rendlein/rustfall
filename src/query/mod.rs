#![allow(dead_code)]
use crate::card;

pub struct Query {
    string: String
}

impl Query {
    pub fn new(req: String) -> Self {
        let query = Query {
            string: req
        };

        query.format()
    }

    fn format(&mut self) {
        self.string = self.string.replace(":", "%3A")
            .replace("=", "%3D")
            .replace(" ", "+");
    }
}