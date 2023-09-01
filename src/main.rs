//! src/main.rs

use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:0")?;
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    print!("\n\t\tRunning on http://localhost:{}\n\n", port);
    run(listener)?.await
}
