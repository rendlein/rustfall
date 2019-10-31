#![allow(dead_code)]
#[test]
fn new_card_list() {
    use crate::card;
}

#[test]
fn print_card_list() {
    use crate::card;
}

#[test]
fn query_test_new() {
    use crate::query::Query;
    let q = Query::new("t:creature power=3".to_string());

    assert_eq!(q.string, "https://api.scryfall.com/cards/search?t%3Acreature+power%3D3");
}
