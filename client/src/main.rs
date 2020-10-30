use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

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

    let mut content = Vec::new();
    File::open(&args[1])
        .expect("Failed to open file")
        .read_to_end(&mut content)
        .expect("Failed to convert to string");

    let header = format!(
        "{{\"name\" : \"{}\",\n\"size\" : {} }}",
        &args[1],
        &content.len()
    );

    stream
        .write(header.as_bytes())
        .expect("Failed to send header");
    println!("[+] Header sent sucessfully");

    let mut counter = 0usize;

    loop {
        let mut buf: [u8; 4096] = [0; 4096];
        if counter + 4096 < content.len() {
            buf.copy_from_slice(&content[counter..counter + 4096]);
            stream.write(&buf).expect("Failed to send part of file");
            counter += 4096;
        } else {
            stream
                .write(&content[counter..content.len()])
                .expect("Failed to send part of file");
            break;
        }
    }
    println!("[+] File sent successfully !");
}
