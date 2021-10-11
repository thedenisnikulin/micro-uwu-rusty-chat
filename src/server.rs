use log::*;
use std::io::{BufRead, BufReader, Result};
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::misc::{Broadcast, ClientHandle, MessageResult, Peer};

pub struct Server {
    inner: Arc<ServerInner>,
}

struct ServerInner {
    listener: TcpListener,
    peers: Mutex<Vec<Arc<Peer>>>,
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
                peers: Mutex::new(Vec::<Arc<Peer>>::new()),
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

            if let Err(_) = msg_result.value {
                self.inner
                    .peers
                    .lock()
                    .unwrap()
                    .retain(|x| x.addr != msg_result.sender.addr);
            };

            let msg = match msg_result.value {
                Ok(ref v) => v,
                Err(ref v) => v,
            };

            self.inner
                .peers
                .lock()
                .unwrap()
                .iter()
                .filter(|x| x.addr != msg_result.sender.addr)
                .collect::<Vec<_>>()
                .broadcast(msg);
        }
    }

    fn read_from_client(client: Arc<Peer>, tx: Sender<MessageResult>) {
        let mut buf_reader = BufReader::new(&client.stream);
        loop {
            let mut buf = String::new();
            let value = match buf_reader.read_line(&mut buf) {
                Ok(n) if n < 1 => Err(format!("🌙 {} покидает нас, грусть :с 🌙\n", client.name)),
                Err(e) => Err(format!("🌙 {} покидает нас, технические шоколадки: {} 🌙\n", client.name, e)),
                Ok(_) => {
                    Ok(format!("[🍃{}🍃]: {}", client.name, buf))
                }
            };

            let is_err = value.is_err();

            tx.send(MessageResult::new(value, &client)).unwrap();

            if is_err {
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
            let mut buf_reader = BufReader::new(&client_accepted.0);

            // read name
            let mut name = String::new();
            buf_reader.read_line(&mut name).unwrap();
            name.pop(); // pop '\n'

            // save to self.inner.peers
            peers.push(Arc::new(Peer::new(
                client_accepted.0,
                client_accepted.1,
                name,
            )));

            // clone & send by tx
            let peer_ref = Arc::clone(peers.last().unwrap());
            let tx = sender.clone();
            tx.send(MessageResult::new(
                Ok(format!("🍍 У нас новенький, поприветствуйте {} 🍍\n", peer_ref.name)),
                &peer_ref,
            ))
            .unwrap();

            // spawn new reader thread
            thread::spawn(move || Self::read_from_client(peer_ref, tx));
        }
    }
}
