pub mod server;
pub mod client;

pub struct Config {
    pub server_mode: bool,
    pub ip: String,
    pub port: String,
}

pub fn help() {
    let help_text = r#"
================
CHET 5 HELP MENU
================

usage: chet5 <mode> <ip> <port>

mode:
    -c: Client mode
    -s: Server mode

ip: The IPv4 address to host on or connect to.

port: The port to host on or connect to.
    "#;
    println!("{}", help_text);
}