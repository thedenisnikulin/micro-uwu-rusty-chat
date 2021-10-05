use log::*;
use std::io::{BufRead, BufReader, Read, Result};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::process::exit;
use std::sync::{mpsc, Arc};
use std::thread;

use crate::{Broadcast, ClientHandle};

pub struct Message {
    pub value: String,
    pub sender: Arc<ClientHandle>,
}

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn bind<A>(addr: A) -> Result<Server>
    where
        A: ToSocketAddrs,
    {
        Ok(Server {
            listener: TcpListener::bind(addr)?,
        })
    }

    pub fn handle_clients(&mut self) {
        println!("create");
        let mut peers = Vec::<Arc<ClientHandle>>::with_capacity(5);
        let (sender, receiver) = mpsc::channel::<Message>();

        for i in (1..=2).rev() {
            println!("Waiting for {} people to connect...", i);
            let sock = self.listener.accept().unwrap();
            peers.push(Arc::new(sock));

            let peer_ref = Arc::clone(peers.last().unwrap());
            let tx = sender.clone();
            thread::spawn(move || Self::read_from_client(peer_ref, tx));
        }

        loop {
            let msg = receiver.recv().unwrap();
            debug!("received from reader thread: {}", msg.value);
            if msg.value.len() < 1 {
                peers.retain(|x| &x.1 != &msg.sender.1);
            }
            peers
                .iter()
                .filter(|x| x.1 != msg.sender.1)
                .collect::<Vec<_>>()
                .broadcast(&msg.value);
        }
    }

    fn read_from_client(client: Arc<ClientHandle>, tx: mpsc::Sender<Message>) {
        let mut buf_reader = BufReader::new(&client.0);
        loop {
            let mut buf = String::new();
            let bytes_read = buf_reader.read_line(&mut buf).unwrap();
            debug!("read from client: {}, len: {}", buf, buf.len());
            tx.send(Message {
                value: buf,
                sender: {
                    let client = Arc::clone(&client);
                    client
                },
            })
            .unwrap_or_else(|err| {
                eprintln!("[Server::read_from_client] err sending: {}", err);
                panic!();
            });
            if bytes_read < 1 {
                break;
            };
        }
    }
}
