use chet5::client;
use chet5::server;
use chet5::Config;
use std::env;
use std::process;

fn main() {
    let mut args = env::args().into_iter();

    let mut config: Config = Config {
        server_mode: false,
        ip: String::from(""),
        port: String::from(""),
    };

    args.next();

    if let Some(arg) = args.next() {
        if arg == String::from("-s") {
            config.server_mode = true;
        } else if arg == String::from("-c") {
            config.server_mode = false;
        } else {
            println!("Unknown argument {}.", arg);
            process::exit(1);
        }
    } else {
        chet5::help();
        process::exit(0);
    }

    if let Some(arg) = args.next() {
        config.ip = arg;
    } else {
        println!("No IP address was supplied as an argument.\nRun without arguments for help menu.");
        process::exit(1);
    }

    if let Some(arg) = args.next() {
        config.port = arg;
    } else {
        println!("No port was supplied as an argument.\nRun without arguments for help menu.");
        process::exit(1);
    }

    if (config.server_mode) {
        server::server(config);
    } else {
        client::client(config);
    }

}