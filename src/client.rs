use crate::AskInput;
use log::*;
use std::{io::{self, BufRead, BufReader, Read, Write}, net::*};

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
            debug!("in client, recv: {}, len: {}", buf, buf.len());
            println!("{}", buf);
        }
    }

    pub fn input_and_send(&self) {
        let mut stdin = io::stdin();

        loop {
            let mut buf = String::new();
            stdin.ask_input("you: ", &mut buf).unwrap();
            debug!("in client, input msg: {}", buf);
            (&self.stream).write_all(&buf.as_bytes()).unwrap();
        }
    }
}