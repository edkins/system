use std::fs;
use std::path::Path;
use std::os::unix::net::UnixListener;
use std::env::home_dir;
//use std::io::{Acceptor,Listener};

static SOCKET_PATH: &'static str = "nuance.sock";

fn main() {
    let socket = home_dir().unwrap().join(Path::new(SOCKET_PATH));

    // Delete old socket if necessary
    if socket.exists() {
        fs::remove_file(&socket).unwrap();
    }

    // Bind to socket
    let listener = UnixListener::bind(&socket).unwrap();
    println!("Server started, waiting for clients");

    for stream in listener.incoming() {
        println!("Client connected!");
    }
}
