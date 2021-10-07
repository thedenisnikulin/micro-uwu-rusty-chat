use crate::misc::AskInput;
use std::{
    io::{self, BufRead, BufReader, Write},
    net::*,
};

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn connect<A>(addr: A) -> io::Result<Client>
    where
        A: ToSocketAddrs,
    {
        Ok(Client {
            stream: TcpStream::connect(addr)?,
        })
    }

    pub fn recv(&self) {
        let mut buf_reader = BufReader::new(&self.stream);

        loop {
            let mut buf = String::new();
            buf_reader.read_line(&mut buf).unwrap();
            print!("\r\x1b[K");
            print!("челик: {}", buf);
        }
    }

    pub fn input_and_send(&self) {
        let mut stdin = io::stdin();

        loop {
            let mut buf = String::new();
            stdin.ask_input("", &mut buf).unwrap();
            (&self.stream).write_all(&buf.as_bytes()).unwrap();
        }
    }
}
