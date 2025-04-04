#![allow(dead_code)]

use window::create_window;

mod db;
// mod server;
mod window;

fn main() {
    window::create_window();
    println!("Hello, world!");
    //start_server();
}

#[derive(Default, Debug, Clone)]
struct Card {
    pub name: String,
    pub keyruneCode: String,
}