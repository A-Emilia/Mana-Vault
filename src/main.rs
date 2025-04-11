#![allow(dead_code)]


mod db;
// mod server;
mod window;
mod model;
mod server;
mod client;
mod com;

fn main() {
    //window::create_window();
    println!("Hello, world!");
    server::start_server();
}

// These functions are given for hw 11/04, no need to implement them.
fn push_card(input: model::Card) { todo!() }
fn get_card(/* Get creative with Arguments */) -> model::Card { todo!() }