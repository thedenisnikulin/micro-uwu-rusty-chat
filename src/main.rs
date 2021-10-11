use clap;
use log::*;
use mean_capybara::client::Client;
use mean_capybara::misc::AskInput;
use mean_capybara::server::Server;
use std::io::{self, Write};
use std::sync::Arc;
use std::thread;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut stdin = io::stdin();
    let client;
    let mut server;

    let options = clap::App::new("")
        .arg(
            clap::Arg::with_name("server")
                .short("s")
                .long("server")
                .help("Launches the program in server mode.")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    if options.is_present("server") {
        let mut addr = String::new();

        stdin.ask_input("Enter the addr:port to bind on: ", &mut addr)?;

        server = Server::bind(&addr[..addr.len() - 1])?;

        server.handle_clients();

        Ok(())
    } else {
        let mut name = String::new();
        let mut addr = String::new();

        println!("🌸✨ Приветик! >-< 🌸✨");

        stdin.ask_input("✨ твоё имечко - ", &mut name)?;
        stdin.ask_input("✨ куда мне подключиться (адрес:порт) - ", &mut addr)?;

        client = Arc::new(Client::connect(&addr[..addr.len() - 1])?);

        println!("🦀 Приятного общения :з 🦀");

        (&client.stream).write_all(&name.as_bytes())?;

        let handle_recv = thread::spawn({
            let client = Arc::clone(&client);
            move || client.recv()
        });

        let handle_send = thread::spawn({
            let client = Arc::clone(&client);
            move || client.input_and_send()
        });

        handle_recv.join().unwrap()?;
        handle_send.join().unwrap()?;
        Ok(())
    }
}
