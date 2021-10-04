use mean_capybara::client::{self, Client};
use mean_capybara::server::Server;
use mean_capybara::AskInput;
use std::io::Write;
use std::io::{self, Read};
use std::net::{self, SocketAddr, TcpStream};
use std::process::exit;
use std::sync::Arc;
use std::thread;
use log::*;

// TODO do smth with all unwraps

fn main() {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    let mut client;
    let mut server;

    env_logger::init();

    stdin
        .ask_input_cut("[client] Enter the port to connect to: ", &mut buf)
        .unwrap();
    debug!("in main, buf: {}, len: {}", buf, buf.len());
    if let Ok(c) = Client::connect(format!("127.0.0.1:{}", buf)) {
        client = Arc::new(c);

        let handle_recv = thread::spawn({
            let client = Arc::clone(&client);
            move || client.recv()
        });

        let handle_send = thread::spawn({
            let client = Arc::clone(&client);
            move || client.input_and_send()
        });

        handle_recv.join().unwrap();
        handle_send.join().unwrap();
    } else {
        // TODO handle error somehow (print it?)
        buf.clear();

        stdin
            .ask_input_cut("[server] Enter the port to bind on: ", &mut buf)
            .unwrap();

        debug!("in main, buf: {}, len: {}", buf, buf.len());
        server = Server::bind(format!("127.0.0.1:{}", buf)).unwrap_or_else(|e| {
            println!("Bye-bye!, {}", e);
            exit(0);
        });

        server.handle_clients();
    }
}
