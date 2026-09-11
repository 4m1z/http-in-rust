mod http;
use http::Http;


fn main() {
    let http_server = Http::new(String::from("127.0.0.1"), 8080);
    match http_server {
        Ok(server) => {
            server.listen();
        }
        Err(err) => {
            println!("error in running server {}", err.message)
        }
    };
}
