use std::{
    borrow::Borrow,
    io::{self, Write},
    net::{SocketAddr, TcpStream},
    sync::Arc,
};

pub type ClientHandle = (TcpStream, SocketAddr);

pub struct MessageResult {
    pub value: std::result::Result<String, String>,
    pub sender: Arc<ClientHandle>,
}

impl MessageResult {
    pub fn new(
        value: std::result::Result<String, String>,
        sender: &Arc<ClientHandle>,
    ) -> MessageResult {
        MessageResult {
            value,
            sender: {
                let sender = Arc::clone(&sender);
                sender
            },
        }
    }
}

pub trait Broadcast {
    fn broadcast(&self, msg: &str);
}

impl<B: Borrow<Arc<ClientHandle>>> Broadcast for Vec<B> {
    fn broadcast(&self, msg: &str) {
        for peer in self {
            (&peer.borrow().0 as &TcpStream)
                .write_all(msg.as_bytes())
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
        io::stdout().flush()?;
        self.read_line(&mut buf)
    }
}
