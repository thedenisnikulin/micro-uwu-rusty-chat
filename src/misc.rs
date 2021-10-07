use std::{
    borrow::Borrow,
    io::{self, Write},
    net::{SocketAddr, TcpStream},
    sync::Arc,
};

pub type ClientHandle = (TcpStream, SocketAddr);

pub trait Broadcast {
    fn broadcast(&self, msg: &str);
}

impl<B: Borrow<Arc<ClientHandle>>> Broadcast for Vec<B> {
    fn broadcast(&self, msg: &str) {
        for peer in self {
            (&peer.borrow().0 as &TcpStream)
                .write(msg.as_bytes())
                .unwrap();
        }
    }
}

pub trait AskInput {
    fn ask_input(&mut self, input_msg: &str, buf: &mut String) -> io::Result<usize>;
}

impl AskInput for io::Stdin {
    fn ask_input(&mut self, input_msg: &str, mut buf: &mut String) -> io::Result<usize> {
        print!("{}", input_msg);
        io::stdout().flush().unwrap();
        self.read_line(&mut buf)
    }
}
