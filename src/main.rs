use log::*;
use mean_capybara::client::Client;
use mean_capybara::misc::AskInput;
use mean_capybara::server::Server;
use std::io;
use std::process::exit;
use std::sync::Arc;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut buf = String::new();
    let mut stdin = io::stdin();
    let client;
    let mut server;

    stdin.ask_input("[client] Enter the port to connect to: ", &mut buf)?;
    debug!("in main, buf: {}, len: {}", buf, buf.len());
    if let Ok(c) = Client::connect(format!("127.0.0.1:{}", &buf[..buf.len() - 1])) {
        client = Arc::new(c);

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
    } else {
        buf.clear();

        stdin.ask_input("[server] Enter the port to bind on: ", &mut buf)?;

        debug!("in main, buf: {}, len: {}", buf, buf.len());
        server = Server::bind(format!("127.0.0.1:{}", &buf[..buf.len() - 1]))?;

        server.handle_clients();

        Ok(())
    }
}
