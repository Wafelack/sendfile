use serde_json::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::net::TcpListener;

fn main() {
    let localaddr = "0.0.0.0:25368";

    const BUFSIZE: usize = 4096;

    let socket = TcpListener::bind(localaddr).expect("Failed to bind to ip");

    loop {
        let mut buf = [0; BUFSIZE];

        for stream in socket.incoming() {
            let mut stream = stream.unwrap();

            let number_of_bytes = stream.read(&mut buf).expect("Failed to receive bytes");

            let v: Value =
                serde_json::from_slice(&buf[..number_of_bytes]).expect("Failed to parse header");

            if let Value::String(s) = &v["name"] {
                if let Value::Number(n) = &v["size"] {
                    let mut f = File::create(s).expect("Failed to create file");

                    let size = n.as_u64().unwrap_or(0);
                    let mut bytes_received = 0usize;

                    let mut content: Vec<u8> = Vec::new();

                    while bytes_received < size as usize {
                        let mut tempbuf = [0; BUFSIZE];
                        let b_amount = stream.read(&mut tempbuf).expect("Failed to receive bytes");

                        for e in &tempbuf[..b_amount] {
                            content.push(*e);
                        }

                        bytes_received += b_amount;
                    }
                    f.write_all(content.as_slice())
                        .expect("Failed to write in file");
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }
}
