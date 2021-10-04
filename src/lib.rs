pub mod client;
pub mod server;

use std::io::Write;
use std::io::{self, Read};
use std::net::SocketAddr;
use std::net::TcpStream;
use std::sync::Arc;

type ClientHandle = (TcpStream, SocketAddr);

pub trait Broadcast {
    fn broadcast(&mut self, msg: &str);
}

// TODO make impl more general
impl Broadcast for Vec<Arc<ClientHandle>> {
    fn broadcast(&mut self, msg: &str) {
        for peer in self {
            (&peer.0).write(msg.as_bytes()).unwrap();
        }
    }
}

pub trait AskInput {
    fn ask_input(&mut self, input_msg: &str, buf: &mut String) -> io::Result<usize>;
    fn ask_input_cut(&mut self, input_msg: &str, buf: &mut String) -> io::Result<usize>;
}

impl AskInput for io::Stdin {
    fn ask_input(&mut self, input_msg: &str, mut buf: &mut String) -> io::Result<usize> {
        print!("{}", input_msg);
        io::stdout().flush().unwrap();
        let ret = self.read_line(&mut buf);
        //buf.pop();
        ret
    }

    // sry for this gonna refactor just lazy
    fn ask_input_cut(&mut self, input_msg: &str, mut buf: &mut String) -> io::Result<usize> {
        print!("{}", input_msg);
        io::stdout().flush().unwrap();
        let ret = self.read_line(&mut buf);
        buf.pop();
        ret
    }
}
