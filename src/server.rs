use log::*;
use std::io::{BufRead, BufReader, Result};
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::{Broadcast, ClientHandle};

pub struct Message {
    pub value: String,
    pub sender: Arc<ClientHandle>,
}

pub struct Server {
    inner: Arc<ServerInner>,
}

struct ServerInner {
    listener: TcpListener,
    peers: Mutex<Vec<Arc<ClientHandle>>>,
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Server {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Server {
    pub fn bind<A>(addr: A) -> Result<Server>
    where
        A: ToSocketAddrs,
    {
        Ok(Server {
            inner: Arc::new(ServerInner {
                listener: TcpListener::bind(addr)?,
                peers: Mutex::new(Vec::<Arc<ClientHandle>>::new()),
            }),
        })
    }

    pub fn handle_clients(&mut self) {
        let (sender, receiver) = mpsc::channel::<Message>();

        // Spawn acceptor thread
        thread::spawn({
            let mut local_self = self.clone();
            move || local_self.accept_connections(sender)
        });

        // Broadcast data received from reader thread
        loop {
            let msg = receiver.recv().unwrap();
            debug!(
                "Received from reader thread: {}, sender: {}",
                msg.value, msg.sender.1
            );
            if msg.value.len() < 1 {
                self.inner.peers.lock().unwrap()
                    .retain(|x| &x.1 != &msg.sender.1);
                debug!("Client dropped: {}", msg.sender.1);
            }
            self.inner.peers.lock().unwrap()
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

    fn accept_connections(&mut self, sender: Sender<Message>) {
        loop {
            let client_accepted = self.inner.listener.accept().unwrap();
            debug!("Client accepted: {}", client_accepted.1);
            let mut peers = self.inner.peers.lock().unwrap();
            peers.push(Arc::new(client_accepted));

            let peer_ref = Arc::clone(peers.last().unwrap());
            let tx = sender.clone();
            thread::spawn(move || Self::read_from_client(peer_ref, tx));
        }
    }
}
