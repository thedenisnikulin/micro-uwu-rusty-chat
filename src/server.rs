use std::io::{BufRead, BufReader, Read, Result};
use std::net::{TcpListener, ToSocketAddrs};
use std::process::exit;
use std::sync::{mpsc, Arc};
use std::thread;
use log::*;

use crate::{Broadcast, ClientHandle};

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
        let (sender, receiver) = mpsc::channel::<String>();

        for i in (1..=2).rev() {
            println!("Waiting for {} people to connect...", i);
            let sock = self.listener.accept().unwrap();
            peers.push(Arc::new(sock));

            let peer_ref = Arc::clone(peers.last().unwrap());
            let tx = sender.clone();
            thread::spawn(move || Self::read_from_client(peer_ref, tx));
            // TODO
        }

        loop {
            // TODO
            debug!("ready to receive from threads!");
            let recvd = receiver.recv().unwrap();
            debug!("received from reader thread: {}, len: {}", recvd, recvd.len());
            peers.broadcast(&recvd);
        }
    }

    fn read_from_client(client: Arc<ClientHandle>, tx: mpsc::Sender<String>) {
        let mut buf_reader = BufReader::new(&client.0);
        loop {
            let mut buf = String::new();
            debug!("read from client, waiting...");
            buf_reader.read_line(&mut buf).unwrap();
            debug!("read from client: {}, len: {}", buf, buf.len());
            tx.send(buf).unwrap_or_else(|err| {
                eprintln!("[Server::read_from_client] err sending: {}", err);
                panic!();
            });
        }
    }
}
