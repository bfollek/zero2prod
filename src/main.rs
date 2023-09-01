//! src/main.rs

use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");
    let _ = listener.local_addr().unwrap().port();
    run(listener)?.await
}
