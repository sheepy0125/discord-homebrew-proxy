#![allow(non_upper_case_globals)]

/// Proxy for the 3DS Discord client
/* Import */
use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use serenity::{
    async_trait,
    json::json,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use reqwest;

/* Constants */
const BUFFER_SIZE: usize = 1024 as usize;
const AVATAR_URL: &str =
    "https://cdn.discordapp.com/avatars/971197852493692928/544fb209e3b265a59befde0f5a884d25.webp";

/* Globals */
/// The current message to send to the 3DS
static mut global_message: String = String::new();
static mut streams: Vec<TcpStream> = vec![];
static mut webhook_url: String = String::new();

/* Bot handlers */
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, message: Message) {
        let formatted_message = format!(
            "{}:{} -> {}",
            message.author.name, message.author.discriminator, message.content
        );
        unsafe {
            if global_message.is_empty() {
                global_message = formatted_message;
            } else {
                global_message = format!("{}\n{}", global_message, formatted_message);
            }
        }
        unsafe {
            println!("Message received: {}", global_message);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} online!", ready.user.name);
    }
}

/* Functions */
/// Handle data sent and return what to write
unsafe fn handle_data(data: &String) -> Vec<String> {
    let mut responses = Vec::new();

    // Hello handshake
    if data.starts_with("HELLO3DS") {
        println!("Handshake received");
        responses.push("HELLO3DS".to_string());
    }

    // Send message
    if data.starts_with("SEND") {
        let message = data[("SEND".len())..].to_string();
        println!("Message received: {}", message);

        let client = reqwest::blocking::Client::new();
        client
            .post(&webhook_url)
            .json(&json!({ "username": "3DSXL", "content": message, "avatar_url": AVATAR_URL }))
            .send()
            .unwrap();

        responses.push("SENT".to_string());
    }

    // Get message
    if data.starts_with("GET") {
        if global_message.len() > 0 {
            let message = global_message.clone();
            println!("Sending message: {}", message);
            responses.push(format!("MESSAGE{}", message));
            global_message.clear();
        } else {
            println!("No message to send");
            responses.push("NONE".to_string());
        }
    }

    return responses;
}

/// Handle a client
unsafe fn handle_client(mut stream: TcpStream) {
    streams.push(stream.try_clone().unwrap());
    let mut data = [0 as u8; BUFFER_SIZE];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                false
            } else {
                let data_str = String::from_utf8_lossy(&data[0..size]).to_string();
                println!("Received: {}", &data_str);
                let responses = handle_data(&data_str);
                for response in responses {
                    println!("Sending: {}", &response);
                    stream.write(response.as_bytes()).unwrap();
                }
                true
            }
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {:?}",
                stream.peer_addr()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            streams.retain(|x| x.peer_addr().unwrap() != stream.peer_addr().unwrap());
            false
        }
    } {}
}

/// Run
#[tokio::main]
async fn main() {
    // Start listener
    let listener = TcpListener::bind("0.0.0.0:7000").unwrap();
    // Accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 7000");
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => unsafe {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || handle_client(stream));
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        // Cleanup
        drop(listener);
    });

    // Setup webhook
    unsafe {
        webhook_url = env::var("WEBHOOK_URL").expect("Expected a webhook URL in the environment");
    }

    // Start bot
    println!("Getting token...");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("Starting bot...");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating bot");

    if let Err(status) = client.start().await {
        println!("Error starting bot: {:?}", status);
    }
}
