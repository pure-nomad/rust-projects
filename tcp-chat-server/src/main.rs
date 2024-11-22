mod client;
mod server;

pub use crate::client::client_handler;
pub use crate::server::server_handler;
use std::env::{self};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        println!("Options: server OR client");
        for arg in args.iter() {
            println!("{}",arg);
        }
        std::process::exit(0);
    }

    match args[2].as_str() {
        "server" => {let _ = server_handler::start_server();},
        "client" => {let _ = client_handler::start_client();},
        _ => println!("Wrong option. Please use server OR client"),
    }
}