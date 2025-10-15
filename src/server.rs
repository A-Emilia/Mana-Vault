use std::{
    io::{BufReader, Lines, prelude::*},
    net::{TcpListener, TcpStream},
};

use crate::model::Card;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let mut iter = buf_reader.lines().map(|result: Result<String, std::io::Error>| result.unwrap());
    
    let http_request: Vec<_> = iter
        .by_ref() 
        .take_while(|line| !line.is_empty())
        .collect();

    let http_body = iter
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!("Request: {http_request:#?}");
    println!("Request Body: {:?}", http_body);

    //println!("Vec index 6: {:#?}", http_request.iter().nth(6));

    let test_card = Card::new(
        "69".to_owned(),
        "69".to_owned(),
        "Jodah the Sexist".to_owned(),
        "uwu".to_owned(),
        "Commits sexism".to_owned(),
        None,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );

    let json = serde_json::to_string(&test_card).unwrap();

    if http_request
        .iter()
        .nth(0)
        .is_some_and(|x| x.contains("GET"))
    {
        stream.write(json.as_bytes()).unwrap();
    }
}

#[test]
fn test_server() {
    start_server();
}
