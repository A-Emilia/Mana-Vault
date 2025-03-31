#![allow(dead_code)]

mod db;
mod server;
mod window;

fn main() {
    let window_thread = std::thread::spawn(window::create_window);
    println!("Hello, world!");
    //start_server();

    match window_thread.join() {
        Ok(_) => println!("Window successfully closed.. Shutting down application."),
        Err(e) => eprintln!("Error in Windowing thread! Error:\n{:?}", e),
    };
    
}

#[derive(Default, Debug, Clone)]
struct Card {
    pub name: String,
}