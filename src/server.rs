use std::net::{TcpListener, ToSocketAddrs};
use std::io::{Result};
use std::sync::{Arc, mpsc};
use std::thread;

use crate::{Broadcast, ClientHandle};

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn bind<A>(addr: A) -> Result<Server>
    where
        A: ToSocketAddrs
    {
        Ok(Server { listener: TcpListener::bind(addr)? })
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
            let recvd = receiver.recv().unwrap();
            peers.broadcast(&recvd);
        }
    }

    fn read_from_client(client: Arc<ClientHandle>, tx: mpsc::Sender<String>) {

    }
}
