use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    const BUFSIZE: usize = 4096;

    if args.len() != 3 {
        println!("Usage : filet <filename> <dest>");
        std::process::exit(65);
    }

    if !Path::new(&args[1]).exists() {
        println!("File not found");
        std::process::exit(25);
    }

    let mut stream = TcpStream::connect(&args[2].trim())
        .expect("Cannot connect to ip. Verify if the server is installed on the target machine");

    println!("[+] Successfully connected to {}", &args[2]);

    let f = File::open(&args[1]).expect("Failed to open file");

    let mut reader = BufReader::with_capacity(BUFSIZE, f);
    let len = std::fs::metadata(&args[1]).unwrap().len() as usize;

    let header = format!(
        "{{\"name\" : \"{}\",\n\"size\" : {} }}",
        &args[1],
        std::fs::metadata(&args[1]).unwrap().len()
    );

    stream
        .write(header.as_bytes())
        .expect("Failed to send header");
    println!("[+] Header sent sucessfully");

    let mut counter = 0usize;

    loop {
        let buf = reader.fill_buf().unwrap();

        if buf.is_empty() {
            break;
        }
        if counter >= len {
            break;
        }
        let size = stream.write(buf).unwrap();

        reader.consume(size);

        counter += size;
        /*
        if counter + BUFSIZE < len {
            buf.copy_from_slice(&content[counter..counter + BUFSIZE]);
            stream.write(&buf).expect("Failed to send part of file");
            counter += BUFSIZE;
        } else {
            stream
                .write(&content[counter..content.len()])
                .expect("Failed to send part of file");
            break;
        }*/
    }
    println!("[+] File sent successfully !");
}
