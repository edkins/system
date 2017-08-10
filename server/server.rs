
use std::env::home_dir;
use std::fs;
use std::io::{BufRead,BufReader};
use std::path::Path;
use std::os::unix::net::{UnixListener,UnixStream};
use std::thread;

static SOCKET_PATH: &'static str = "nuance.sock";

fn handle_client(stream: UnixStream) {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();

    reader.read_line(&mut buffer);
    println!("Client connected {}", buffer);
}

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
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Connection error {}", err);
            }
        }
    }
}
