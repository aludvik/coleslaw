extern crate zmq;

use std::process::{Command};
use std::collections::HashMap;

struct Channel {
    socket: zmq::Socket,
}

enum Endpoint {
    Bind(String),
    Connect(String),
}

impl Channel {
    pub fn new(ctx: zmq::Context, endpoint: Endpoint) -> Self {
        let socket = ctx.socket(zmq::PAIR).unwrap();
        match endpoint {
            Endpoint::Bind(bind) => socket.bind(&bind).unwrap(),
            Endpoint::Connect(connect) => socket.connect(&connect).unwrap(),
        };
        Channel{ socket }
    }

    pub fn send<T: AsRef<[u8]>>(&self, data: T) {
        self.socket.send(data.as_ref(), 0).unwrap();
    }

    pub fn recv(&self) -> Vec<u8> {
        self.socket.recv_bytes(0).unwrap()
    }
}

// -- Rust interface --
pub struct Store {
    store: HashMap<i32, i32>,
}

impl Store {
    pub fn new() -> Self {
        Store{ store: HashMap::new() }
    }

    pub fn put(&mut self, key: i32, val: i32) {
        self.store.insert(key, val);
    }

    pub fn get(&self, key: i32) -> Option<&i32> {
        self.store.get(&key)
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        // Print to stdout so we can manually confirm the memory is freed
        // Note that the fields of Store are still dropped automatically
        // Trying to do this is a compiler error:
        // ::std::mem::drop(self.store); <- Try uncommenting this
        println!("dropping store in rust");
    }
}

fn main() {
    let ep = "ipc://test";

    let mut python = Command::new("python3")
        .arg("python/main.py")
        .arg(ep)
        .spawn()
        .expect("Failed to start python subprocess");

    let mut store = Store::new();

    let ctx = zmq::Context::new();
    let chan = Channel::new(ctx, Endpoint::Bind(ep.into()));
    loop {
        let recvd = String::from_utf8(chan.recv()).unwrap();
        println!("Received '{}' from Python", recvd);
        let command: Vec<&str> = recvd.split(" ").collect();
        match command.len() {
            1 => {
                assert!(command[0] == "shutdown");
                break;
            },
            2 => {
                assert!(command[0] == "get");
                let rep = match store.get(command[1].parse().unwrap()) {
                    Some(got) => format!("{}", got),
                    None => String::from(""),
                };
                chan.send(rep);
            },
            3 => {
                assert!(command[0] == "put");
                store.put(command[1].parse().unwrap(), command[2].parse().unwrap());
            },
            _ => {
                panic!("Received unexpected command")
            },
        }
    }
    python.wait().unwrap();
    }
