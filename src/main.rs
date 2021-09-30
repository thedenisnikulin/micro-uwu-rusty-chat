use std::io::Write;
use std::io::{self, Read};
use std::net::{self, SocketAddr, TcpStream};
use std::process::exit;
use mean_capybara::AskInput;
use mean_capybara::client::{self, Client};
use mean_capybara::server::Server;

// TODO do smth with all unwraps

fn main() {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    let mut client: Client;
    let mut server: Server;

    stdin.ask_input("[client] Enter the port to connect to:", &mut buf).unwrap();
    if let Ok(_) = Client::connect(format!("127.0.0.1:{}", buf)) {
        todo!("recv and send threads");
    } else {
        // TODO handle error somehow (print it?)
        buf.clear();
        stdin.ask_input("[server] Enter the port to bind on:", &mut buf).unwrap();
        server = Server::bind(format!("127.0.0.1:{}", buf)).unwrap_or_else(|_| {
            println!("Bye-bye!");
            exit(0);
        });
        server.handle_clients();
    }
}
