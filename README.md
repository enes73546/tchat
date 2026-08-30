# tchat

A lightweight, zero-dependency terminal chat application built entirely using the Rust standard library (std::net). It supports automated session persistence via a local text file and cross-network communication.

---

## Installation

To download and install the project on your machine, follow these steps:

1. Clone the repository using Git:
   ```bash
   git clone https://github.com
   ```

2. Navigate into the project directory:
   ```bash
   cd tchat
   ```

3. Compile and install the application globally to your system using Cargo:
   ```bash
   cargo install --path . --force
   ```
   The --force flag ensures that any previous versions of tchat on your system are overwritten with the latest build. Once completed, you can run the application globally from any terminal directory using the "tchat" command instead of "cargo run --".

---

## Getting Started

### Authentication (One-Time Login)
tchat uses a local session.txt file to remember who you are so you do not have to log in every time you want to send a message. Log in once by providing your username:

```bash
tchat -login YourUsername
```
This saves your active login state to your directory as 1;YourUsername;.

---

## Hosting a Chat Party on Your IP

To act as the central chat host so your friends can connect, you must run the app in server mode.

### Step 1: Start the Server Host
```bash
tchat -server
```
The program binds to 0.0.0.0:8080, allowing it to listen for traffic coming from both your local computer and the outside network.

### Step 2: Open Your Network (Port Forwarding)
If your friends are in the same house on the same Wi-Fi network, they can connect directly to your local IP address (e.g., 192.168.1.X).

If your friends are connecting over the internet:
1. Find your IPv4 Address and Default Gateway by running "ipconfig" in your Windows command prompt.
2. Log into your home router's admin page using the Default Gateway IP.
3. Locate the Port Forwarding menu.
4. Forward port 8080 (using the TCP protocol) to your machine's local IPv4 Address.

---

## Sending Chat Messages

### For the Host (Testing Locally)
If you are running the server on your own computer, you can test sending messages locally by running:

```bash
tchat -send "Hey, the server is up!"
```
Note: In the source code, the client naturally routes to 127.0.0.1:8080 for local system verification.

### For Your Friends (Connecting to Your Party)
When your friends want to text your server over the internet, they simply need to replace the connection target IP inside src/main.rs with your public IP address (which you can find by searching "what is my IP" on Google).

Once their client points to your public IP, they just type:
```bash
tchat -send "Thanks for hosting the chat party!" -server "YOUR_PUBLIC_IP"
```

---

## Internal Framework Overview
* Session Tracking: Avoids repetitive command arguments by storing states as semicolon-separated raw strings (1;Username;) using standard std::fs operations.
* Network Protocol: Leverages low-level TcpListener stream buffers parsed into human-readable strings via String::from_utf8_lossy.
