use std::io::Write;
use std::io::{self, Read};
use std::net::{self, SocketAddr, TcpStream};
use std::process::exit;
use std::sync::Arc;
use std::thread;
use std::sync::mpsc;

fn main() {
    let stdin = io::stdin();
    let reader = StdinReader::new(stdin);

    let mut buf = String::new();
    if let Ok(_) = reader.n_bytes("[connect] Enter server port: ", &mut buf) {
        connect(format!("127.0.0.1:{}", buf))
    } else {
        buf.clear();
        reader
            .n_bytes("[create] Enter server port: ", &mut buf)
            .unwrap_or_else(|_| {
                println!("Bye-bye!");
                exit(0);
            });
        println!("port: {}, len: {}", buf, buf.len());
        create_server(format!("127.0.0.1:{}", buf));
    }
}

type ClientHandle = (TcpStream, SocketAddr);

fn connect<A>(addr: A)
where
    A: net::ToSocketAddrs,
{
    println!("connect");
    let mut stream = net::TcpStream::connect(addr).unwrap();
    println!("connected");
    let mut buf0 = String::new();
    print!("cmd: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf0).unwrap();
    buf0.pop();
    println!("buf0 is {}", buf0);
    if buf0 == "recv" {
        buf0.clear();
        println!("gonna recv");
        stream.read_to_string(&mut buf0).unwrap();
        println!("received: {}", buf0);
        exit(0);
    };
    println!("gonna write");
    stream.write("no bad words :)".as_bytes()).unwrap();
}

fn create_server<A>(addr: A)
where
    A: net::ToSocketAddrs,
{
    println!("create");
    let listener = net::TcpListener::bind(addr).unwrap();
    let mut peers = Vec::<Arc<ClientHandle>>::with_capacity(5);
    let (sender, receiver) = mpsc::channel::<String>();

    for i in (1..=2).rev() {
        println!("Waiting for {} people to connect...", i);
        let sock = listener.accept().unwrap();
        peers.push(Arc::new(sock));
        let peers_ref = Arc::clone(peers.last().unwrap());
        let tx = sender.clone();
        thread::spawn(move || read_from_client(peers_ref, tx));
    }

    loop {
        let recvd = receiver.recv().unwrap();
        peers.broadcast(&recvd);
    }
}

fn read_from_client(client: Arc<ClientHandle>, tx: mpsc::Sender<String>) {
    loop {
        let mut stream = &client.0;
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();
        tx.send(buf).unwrap();
    }
}

trait Broadcast {
    fn broadcast(&mut self, msg: &str);
}

impl Broadcast for Vec<Arc<ClientHandle>> {
    fn broadcast(&mut self, msg: &str) {
        for peer in self {
            (&peer.0).write(msg.as_bytes()).unwrap();
        }
    }
}

struct StdinReader {
    stdin: io::Stdin,
}

impl StdinReader {
    fn new(stdin: io::Stdin) -> StdinReader {
        StdinReader { stdin }
    }

    fn n_bytes(&self, msg: &str, buf: &mut String) -> Result<(), ()> {
        print!("{}", msg);
        io::stdout().flush();
        match self.stdin.read_line(buf) {
            Ok(v) if v < 2 => Err(()),
            Ok(v) => {
                buf.pop();
                Ok(())
            }
            Err(e) => panic!("io err"),
        }
    }
}
