
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use crate::com::*;

pub fn start_client() {
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    handle_connection(stream);
}

pub fn handle_connection(mut stream: TcpStream) {
    stream.write(b"hello").unwrap();
}


#[test]
fn test_client(){
    start_client();
}