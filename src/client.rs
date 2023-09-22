use crate::Config;
use std::net::TcpStream;
use std::process;
use std::io::{self, Write};

pub fn client(config: Config) {
    let mut stream = match TcpStream::connect(format!("{}:{}", config.ip, config.port)) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to connect to server:\n{}", e);
            process::exit(1);
        }
    };

    println!("Enter messages to send:");
    let mut input = String::from("");

    loop {
        input = String::from("");
        println!("Input before reading: \"{}\"", input);
        if let Ok(_) = io::stdin().read_line(&mut input) {
            println!("Input before comparison: \"{}\"", input);
            println!("\"{}\"", input);
            if input.trim().to_lowercase() == "exit!\n" {
                println!("Exiting program");
                process::exit(0);
            } else {
                match stream.write_all(input.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error sending data:\n{}", e);
                        process::exit(1);
                    }
                };
            }
            println!("Input after comparison: \"{}\"", input);
        }

    }

}