use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let mut is_logged: bool = false;
    let mut user_name: String = String::new();
    let args: Vec<String> = env::args().collect();

    if let Ok(mut file) = File::open("session.txt") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            // Split the file text by the semicolon ";"
            let parts: Vec<&str> = contents.trim().split(';').collect();
            if parts.len() >= 2 {
                if parts[0] == "1" {
                    is_logged = true;
                    user_name = parts[1].to_string();
                }
            }
        }
    }

    if args.len() > 2 && args[1] == "-login" {
        is_logged = true;
        user_name = args[2].to_string();

        let session_data = format!("1;{};", user_name);
        
        if fs::write("session.txt", session_data).is_ok() {
            println!("Logged in successfully as {}. Session saved!", user_name);
        }
    }

    if args.len() > 1 && args[1] == "-server" {
        let listener = TcpListener::bind("0.0.0.0:8080").expect("Could not start server");
        println!("Chat server listening on port 8080...");

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buffer = [0; 512];
                if let Ok(bytes_read) = stream.read(&mut buffer) {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    print!("{}", message);
                }
            }
        }
        return;
    }

    if is_logged {
        if args.len() > 2 && args[1] == "-send" {
            let message_to_send = &args[2];
            
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
                let formatted_message = format!("[{}]: {}\n", user_name, message_to_send);
                let _ = stream.write_all(formatted_message.as_bytes());
            } else {
                println!("Error: Could not connect to the server.");
            }
        } else if args.len() == 1 || (args.len() > 1 && args[1] == "-login") {
            println!("Current active session: {}", user_name);
            println!("Use: cargo run -- -send \"your message\"");
        }
    } else {
        println!("You are not logged in. Use: cargo run -- -login Username");
    }    
}
