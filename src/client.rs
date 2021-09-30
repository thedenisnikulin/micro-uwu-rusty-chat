use std::{io::{self, Read, Write}, net::*};
use crate::{AskInput};

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn connect<A>(addr: A) -> io::Result<Client>
    where
        A: ToSocketAddrs
    {
        Ok(Client { stream: TcpStream::connect(addr)? })
    }

    pub fn recv(&mut self) {
        let mut buf = String::new();

        self.stream.read_to_string(&mut buf).unwrap();

        println!("{}", buf);
    }

    pub fn input_and_send(&mut self) {
        let mut buf = String::new();
        let mut stdin = io::stdin();
        
        stdin.ask_input("you: ", &mut buf).unwrap();

        self.stream.write_all(&buf.as_bytes()).unwrap();
    }
}
