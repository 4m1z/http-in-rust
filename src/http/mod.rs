mod errors;
mod request; 
mod response; 

use std::io::{Read, Write};
use std::net::TcpListener;


use errors::HttpConnectionError;
use request::{Request, Method};
use request::{Response};


pub struct Http {
    host: String,
    pub port: i32,
    listener: TcpListener,
}

impl Http {
    pub fn new(host: String, port: i32) -> Result<Http, HttpConnectionError> {
        match TcpListener::bind(format!("{}:{}", host, port)) {
            Err(err) => {
                return Err(HttpConnectionError::new(host, port, err));
            }
            Ok(listener) => {
                return Ok(Http {
                    host,
                    port,
                    listener,
                });
            }
        }
    }

    pub fn listen(&self) {
        println!("listening on host {} , port {}", self.host,self.port);
        // self.listener.incoming will blocks until tcp completes
        // handshake, then yields one TCP stream per handshake
        // tcp-handshake:
        // TCP needs agreement on sequence numbers/ports before sending data. That is the 3-way handshake,
        // done by OS kernel:
        // 1. Client SYN → server 127.0.0.1:8080
        // 2. Server SYN-ACK → client
        // 3. Client ACK → server
        // Only after (3) listener.incoiming() yields a ready TCP stream
        // when a ready tcp stream is available it's time for read & write
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();

            // 4KB buffer
            let mut buffer = [0; 4096];
            // last index that we read 
            let bytes_read = stream.read(&mut buffer).unwrap();

            let raw_request = String::from_utf8_lossy(&buffer[..bytes_read]);
            // convert the requst to rust rquest struct 
            // parse request 
            let parsed_request = Request::parse(raw_request.clone());

            match parsed_request.method {
                Method::GET  => {
                    let response = self.get(&parsed_request);
                    stream.write_all(response.as_bytes()).unwrap();
                },
                Method::POST => {},
                Method::PUT => {},
                Method::DELETE => {},
                Method::PATCH => {},
            }

            println!("Request:\n{}", request);

            if raw_request.starts_with("GET ") {
            }
        }
    }

    fn get(&self, _request: &Request) -> Response {
        let body = "Hello, world!";

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
                Content-Length: {}\r\n\
                Content-Type: text/plain\r\n\
                Connection: close\r\n\
                \r\n\
                {}",
            body.len(),
            body
        );

        return response;
    }
}
