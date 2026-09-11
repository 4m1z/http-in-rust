use std::io::{Error};

pub struct HttpConnectionError {
    pub host: String,
    pub port: i32,
    pub message: String,
}

impl HttpConnectionError {
    pub fn new(host: String, port: i32, err: Error) -> HttpConnectionError {
        return HttpConnectionError {
            host,
            port,
            message: format!("HTTP connection error: {}", err),
        }
    }
}
