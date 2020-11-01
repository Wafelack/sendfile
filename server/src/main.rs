use std::ffi::OsStr;
use std::fs::remove_file;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::net::TcpListener;
use std::path::Path;

fn main() {
    let localaddr = "0.0.0.0:25368";

    const BUFSIZE: usize = 4096;

    let socket = TcpListener::bind(localaddr).expect("Failed to bind to ip");

    loop {
        let mut buf = [0; 8]; // buffer for header size

        for stream in socket.incoming() {
            let mut stream = stream.unwrap();

            stream
                .read_exact(&mut buf)
                .expect("Failed to receive bytes");

            let header_size: usize = usize::from_be_bytes(buf);

            let mut bufheader: Vec<u8> = Vec::with_capacity(header_size);

            stream
                .read_exact(&mut bufheader)
                .expect("Failed to receive header");

            let header: Vec<&str> = std::str::from_utf8(&bufheader)
                .unwrap()
                .split(';')
                .collect();

            println!("{:?}", header);

            let name = header[0];
            let size = header[1].parse::<usize>().unwrap();

            let path = Path::new(
                Path::new(name)
                    .file_name()
                    .unwrap_or(OsStr::new("temp.txt")),
            );

            if path.exists() {
                match remove_file(path) {
                    Ok(_) => (),
                    Err(_e) => break,
                }
            }
            let mut f = File::create(path).expect("Failed to create file");
            let mut bytes_received = 0usize;

            while bytes_received < size {
                let mut tempbuf = [0; BUFSIZE];
                let b_amount = stream.read(&mut tempbuf).expect("Failed to receive bytes");

                let mut current: Vec<u8> = vec![];
                for e in &tempbuf[..b_amount] {
                    current.push(*e);
                }

                bytes_received += b_amount;
                f.write(current.as_slice()).unwrap();
            }
        }
    }
}
