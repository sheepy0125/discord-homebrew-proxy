/// Proxy for the 3DS Discord client
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

/* Constants */
const BUFFER_SIZE: usize = 1024 as usize;

/* Functions */
/// Handle data sent and return what to write
fn handle_data(data: &String) -> String {
    let mut response = String::new();

    // Hello handshake
    if data.starts_with("HELLO3DS") {
        println!("Handshake received");
        response.push_str("HELLO3DS");
    }

    return response;
}

/// Handle a client
fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; BUFFER_SIZE];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size != 0 {
                let data_str = String::from_utf8_lossy(&data[0..size]).to_string();
                println!("Received: {}", &data_str);
                let response = handle_data(&data_str);
                println!("Sending: {}", &response);
                stream.write(response.as_bytes()).unwrap();
                true
            } else {
                // no data received
                false
            }
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {:?}",
                stream.peer_addr()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

/// Run
fn main() {
    let listener = TcpListener::bind("0.0.0.0:7000").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 7000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
