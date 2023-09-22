use std::net::{TcpListener, TcpStream};
use std::io::Read;
use crate::Config;
use std::process;
use std::thread::{self, JoinHandle};

pub fn server(config: Config) {
    let listener = match TcpListener::bind(format!("{}:{}", config.ip, config.port)) {
        Ok(l) => l,
        Err(e) => {
            println!("Error creating listening server:\n{}", e);
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        let new_stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("Error with incoming connection:\n{}", e);
                process::exit(1);
            }
        };
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        handles.push(thread::spawn(move || {
            handle(new_stream);
        }));
    }

}

fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        println!("{}", String::from_utf8_lossy(&buffer))
    }
}