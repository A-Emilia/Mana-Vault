#![allow(dead_code)]

use rocket::{form::Form, get, launch, post, routes, serde::json::{self, Json}, FromForm, Responder};
use serde::{Deserialize, Serialize};
use mvtg_card::{Card, ManaCost, CardType, SuperType };

mod db;
// mod server;
mod com;
mod server;

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromForm)]
struct CardSearch {
    id: Option<String>,
    oracle_id: Option<String>,
    name: Option<String>,
    set_code: Option<String>,
    text: Option<String>,
    cost: Option<ManaCost>,
    supertype: Option<Vec<SuperType>>,
    card_type: Option<Vec<CardType>>,
    subtype: Option<Vec<String>>,
}

#[get("/cards?<q..>")]
fn card(q: CardSearch) -> Option<Json<Card>> {
    !unimplemented!()
}


static TESTCARDS: [TestCard; 3] = [make_card("Asmoranomardicadaistinaculdacar"), make_card("Jodah, Archmage Eternal"), make_card("Inalla, Archmage Ritualist")];

const fn make_card(input: &'static str) -> TestCard {
    TestCard(input, 2)
}

#[derive(Clone, Serialize, Deserialize)]
struct TestCard (&'static str, i16);


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test_cards/<id>")]
fn test_card(id: usize) -> Option<Json<TestCard>> {
    TESTCARDS.get(id).cloned().map(Json)
}

//#[post("/card/add", data = "<json>")]
//fn add_card(json: Json<TestCard>) -> &'static str {
//    "uwu"
//}

#[get("/cards?<id>")]
fn by_id(id: usize) -> Option<Json<Card>> {
    unimplemented!()
}

#[launch]
fn rocket() -> _ {
    //window::create_window();
    //server::start_server();
    rocket::build()
    .mount("/", routes![index, test_card, card])
}

// These functions are given for hw 11/04, no need to implement them.
fn push_card(input: mvtg_card::Card) {
    todo!()
}
fn get_card(/* Get creative with Arguments */) -> mvtg_card::Card {
    todo!()
}
