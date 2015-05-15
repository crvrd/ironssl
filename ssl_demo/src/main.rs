// Server Demo

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;

// gen certificate via:
// openssl req -new -x509 -key privkey.pem -out cacert.pem -days 1095

fn handle_client(mut stream: TcpStream) {
	// let mut buf : [u8] = [0; 1024];
	// let mut s = "hello".to_string();
	// let mut buf = s.as_bytes();
	// stream.read(buf);
	// println!("{:?}", buf);

	// let s = "hello".to_string();
	// buf = s.as_bytes();
 //    stream.write(buf);
 	println!("Hello World");
}

fn main() {
    // bind listener
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    // accept connections, spawn new threads to handle
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                // connection failed
                println!("incoming: have error {}", e);
            }

            Ok(stream) => {
            	thread::spawn(move|| {
	                // connection succeeded
	                handle_client(stream)
            	}); 
            }
        }
    }

    // close server socket
    drop(listener);
}