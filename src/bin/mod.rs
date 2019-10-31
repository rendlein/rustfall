extern crate rustfall;
extern crate clap;
use rustfall::query::Query;
use clap::{Arg, App};

fn main () {
    let matches = App::new("rustfall")
        .version("1.0.0")
        .author("Jacob Boone")
        .about("Search MTG cards using the Scryfall api")
        .arg(Arg::with_name("TEXT")
            .required(true)
            .multiple(true))
        .get_matches();

    if matches.is_present("TEXT") {
        let mut string: String = "".to_string();
        let text = matches.values_of("TEXT").unwrap();

        for value in text {
            string.push_str(value);
            string.push_str(" ");
        }

        let query = Query::new(string);
        let result = query.run();

        match result {
            Some(mut list) => {
                list.process();
                list.print_list()
            }
            _ => {}
        };
    }
}