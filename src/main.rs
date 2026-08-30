use std::env;
use std::fs::{self, File};
use std::io::{Read, Write, self};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let mut user_name: String = String::from("anonymous");
    let args: Vec<String> = env::args().collect();

    if let Ok(mut file) = File::open("session.txt") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let parts: Vec<&str> = contents.trim().split(';').collect();
            if parts.len() >= 2 && parts[0] == "1" {
                user_name = parts[1].to_string();
            }
        }
    }

    if args.len() > 2 && args[1] == "-login" {
        user_name = args[2].to_string();
        let session_data = format!("1;{};", user_name);
        if fs::write("session.txt", session_data).is_ok() {
            println!("Logged in successfully as {}. Session saved!", user_name);
        }
        return;
    }

    if args.len() > 1 && args[1] == "-server" {
        let listener = TcpListener::bind("0.0.0.0:8080").expect("Could not start server");
        
        let public_ip = match TcpStream::connect("8.8.8.8:53") {
            Ok(stream) => stream.local_addr().map(|addr| addr.ip().to_string()).unwrap_or_else(|_| String::from("0.0.0.0")),
            Err(_) => String::from("0.0.0.0"),
        };
        
        println!("Chat server listening on port 8080...");
        println!("Your Local Network IP: {}", public_ip);
        println!("For cross-city connections, use your Public Router IP.");

        // Wrap the client list in a thread-safe Shared Pointer (Arc + Mutex)
        let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let client_read_clone = stream.try_clone().expect("Failed to clone client stream");
                
                // Add the new client connection to our thread-safe shared list
                let clients_list = Arc::clone(&clients);
                {
                    let mut locked_clients = clients_list.lock().unwrap();
                    locked_clients.push(stream);
                }

                // Spawn a new background thread to handle data sent by this specific client
                let clients_for_thread = Arc::clone(&clients);
                thread::spawn(move || {
                    let mut buffer = [0; 512];
                    let mut reader = client_read_clone;
                    loop {
                        match reader.read(&mut buffer) {
                            Ok(0) => break, 
                            Ok(bytes_read) => {
                                let raw_msg = &buffer[..bytes_read];
                                
                                // Broadcast the incoming message to every single client connected to the city server
                                let mut locked_clients = clients_for_thread.lock().unwrap();
                                locked_clients.retain_mut(|c| {
                                    c.write_all(raw_msg).is_ok()
                                });
                            }
                            Err(_) => break,
                        }
                    }
                });
            }
        }
        return;
    }

    if args.len() > 2 && args[1] == "-join" {
        let target_ip = format!("{}:8080", args[2]);
        println!("Connecting to {} as {}...", target_ip, user_name);

        match TcpStream::connect(&target_ip) {
            Ok(stream) => {
                println!("Connected to the chatroom! Type your message and press Enter.");
                
                let mut stream_read_clone = stream.try_clone().expect("Failed to clone stream for reading");
                thread::spawn(move || {
                    let mut buffer = [0; 512];
                    loop {
                        match stream_read_clone.read(&mut buffer) {
                            Ok(0) => {
                                println!("\nDisconnected from server.");
                                std::process::exit(0);
                            }
                            Ok(bytes_read) => {
                                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                                print!("{}", message);
                                io::stdout().flush().unwrap();
                            }
                            Err(_) => break,
                        }
                    }
                });

                let mut stream_write_clone = stream;
                loop {
                    let mut input = String::new();
                    if io::stdin().read_line(&mut input).is_ok() {
                        let trimmed = input.trim();
                        if !trimmed.is_empty() {
                            let formatted_message = format!("{}: {}\n", user_name, trimmed);
                            if stream_write_clone.write_all(formatted_message.as_bytes()).is_err() {
                                println!("Failed to send message.");
                                break;
                            }
                        }
                    }
                }
            }
            Err(_) => {
                println!("Error: Could not connect to the chat server at {}", target_ip);
            }
        }
        return;
    }

    println!("Usage:");
    println!("  tchat -login Username");
    println!("  tchat -server");
    println!("  tchat -join IP_ADDRESS");
}
