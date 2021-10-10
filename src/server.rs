use log::*;
use std::io::{BufRead, BufReader, Result};
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::misc::{Broadcast, ClientHandle, Message, MessageResult};

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
        let (sender, receiver) = mpsc::channel::<MessageResult>();
        // Spawn acceptor thread
        thread::spawn({
            let mut local_self = self.clone();
            move || local_self.accept_connections(sender)
        });

        // Broadcast data received from reader thread
        loop {
            let msg_result = receiver.recv().unwrap();

            match msg_result.value {
                Ok(msg_value) => self.inner.peers.lock().unwrap()
                    .iter()
                    .filter(|x| x.1 != msg_result.sender.1)
                    .collect::<Vec<_>>()
                    .broadcast(&msg_value),
                Err(msg_value) => {
                    self.inner.peers.lock().unwrap()
                        .retain(|x| &x.1 != &msg_result.sender.1);
                    debug!("{} - {}", msg_result.sender.1, msg_value);
                }
            }
        }
    }

    fn read_from_client(client: Arc<ClientHandle>, tx: mpsc::Sender<MessageResult>) {
        let mut buf_reader = BufReader::new(&client.0);
        loop {
            let mut buf = String::new();
            let value = match buf_reader.read_line(&mut buf) {
                Ok(n) if n < 1 => Err("Client disconnected.".to_string()),
                Err(e) => Err(format!("Receiving message failed: {}", e)),
                Ok(n) => Ok(buf),
            };

            debug!("read from client: {}, len: {}", buf, buf.len());

            tx.send(MessageResult {
                value,
                sender: {
                    let client = Arc::clone(&client);
                    client
                },
            })
            .unwrap();

            if value.is_err() {
                break;
            };
        }
    }

    fn accept_connections(&mut self, sender: Sender<MessageResult>) {
        loop {
            let client_accepted = match self.inner.listener.accept() {
                Ok(client) => client,
                Err(e) => {
                    println!("Error accepting client: {}", e);
                    continue;
                }
            };
            debug!("Client accepted: {}", client_accepted.1);
            let mut peers = self.inner.peers.lock().unwrap();
            peers.push(Arc::new(client_accepted));

            let peer_ref = Arc::clone(peers.last().unwrap());
            let tx = sender.clone();
            thread::spawn(move || Self::read_from_client(peer_ref, tx));
        }
    }
}
