extern crate zmq;

use std::process::{Command};

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

fn main() {
    let mut python = Command::new("python3")
        .arg("../python/main.py")
        .spawn()
        .expect("Failed to start python subprocess");

    let ctx = zmq::Context::new();
    let chan = Channel::new(ctx, Endpoint::Bind("ipc://test".into()));
    for _ in 0..10 {
        chan.send("hello");
        let recvd = chan.recv();
        println!("Received '{}' from Python", String::from_utf8(recvd).unwrap());
    }
    chan.send("shutdown");

    python.wait().unwrap();
}
