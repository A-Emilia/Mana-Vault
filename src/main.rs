#![allow(dead_code)]

use rocket::{form::Form, get, launch, post, routes, serde::json::{self, Json}, FromForm, Responder};
use serde::{Deserialize, Serialize};
use model::Card;
use model::ManaCost;

mod db;
// mod server;
mod client;
mod com;
mod model;
mod server;
mod window;

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

#[get("/cards?<id>&<oracle_id>&<name>&<set_code>&<text>")]
fn card(
    id: Option<&'_ str>,
    oracle_id: Option<&'_ str>,
    name: Option<&'_ str>,
    set_code: Option<&'_ str>,
    text: Option<&'_ str>,
    //cost: Option<ManaCost>,


) -> Option<Json<Card>> {
    !unimplemented!()
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
    rocket::build().mount("/", routes![index, test_card, card])
}

// These functions are given for hw 11/04, no need to implement them.
fn push_card(input: model::Card) {
    todo!()
}
fn get_card(/* Get creative with Arguments */) -> model::Card {
    todo!()
}
