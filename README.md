# tchat

A lightweight, zero-dependency terminal real-time looping chatroom application built entirely using the Rust standard library (std::net and std::thread). It supports automated session persistence via a local text file and continuous multi-user broadcasting.

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

---

## Getting Started

### Authentication (One-Time Login)
Log in once by providing your username. If you skip this step, you will automatically be named "anonymous" inside the chatroom.
```bash
tchat -login YourUsername
```

---

## Hosting a Chat Party on Your IP

To act as the central message broadcaster, run the application in server mode:
```bash
tchat -server
```
The program will display your local network IP address on startup. To allow connections from people across your city, locate your router's Port Forwarding panel and route incoming TCP traffic on port 8080 to that displayed IP address.

---

## Joining an Active Chatroom

To connect to a live chatroom hosted across your city, pass the host's public IP address directly to the join flag:
```bash
tchat -join IP_ADDRESS
```
Once connected, the program opens a continuous network loop. Incoming messages appear automatically on new lines, and you can type your message into the bottom prompt and press Enter to broadcast it live to everyone.
