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

    loop {
        let mut input = String::new();
        
        if let Ok(_) = io::stdin().read_line(&mut input) {
            if input == String::from("exit()") {
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
        }

    }

}