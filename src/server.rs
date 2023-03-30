use std::net::TcpListener;
use std::io::{Write, Read};
use crate::http::*;

use crate::http::Request;
use std::convert::TryFrom;


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}
    
impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }
    
    pub fn run(self, mut handler: impl Handler) {
        println!("Server running on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    println!("Connection from: {}", address);
                    let mut data = [0; 1024];
                    match stream.read(&mut data) {
                        Ok(_) => {
                            let response = match Request::try_from(&data[..]) {
                                Ok(_request) => handler.handle_request(&_request),
                                Err(error) => handler.handle_bad_request(&error),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(error) => {},   
                    }
                },
                Err(error) => println!("{}", error),
            } 
        }
    }
}


