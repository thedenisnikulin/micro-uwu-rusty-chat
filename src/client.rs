use crate::misc::AskInput;
use std::{
    io::{self, BufRead, BufReader, Write},
    net::*,
};

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Client> {
        Ok(Client {
            stream: TcpStream::connect(addr)?,
        })
    }

    pub fn recv(&self) -> io::Result<()> {
        let mut buf_reader = BufReader::new(&self.stream);

        loop {
            let mut buf = String::new();
            buf_reader.read_line(&mut buf)?;
            print!("\r\x1b[K"); // Jump to the beginning of the line
            print!("челик: {}", buf); // '\n' is not used, it comes from buf
        }
    }

    pub fn input_and_send(&self) -> io::Result<()> {
        let mut stdin = io::stdin();

        loop {
            let mut buf = String::new();
            stdin.ask_input("", &mut buf)?;
            (&self.stream).write_all(&buf.as_bytes())?;
        }
    }
}
