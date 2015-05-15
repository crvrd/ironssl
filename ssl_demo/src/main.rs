// Server Demo

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::string::String;

// gen certificate via:
// openssl req -new -x509 -key privkey.pem -out cacert.pem -days 1095

fn handle_client(mut stream: TcpStream) {
	// let mut buf : [u8] = [0];
	// let mut s = "hello".to_string();
	// let mut buf = s.as_bytes();
	// stream.read(&buf);
	// println!("{:?}", buf);

    // stream.write(b"hello");
 	// println!("Hello World");
 	
 // 	stream.write(b"GET / HTTP/1.0\n\n");
	// let response = &mut buf
	// stream.read_to_end(response);
	// println!("{}", response);

 	let mut out_stream = stream.try_clone().unwrap();
 	let mut msg: Vec<u8> = Vec::new();
    {
	    let mut msg_ptr: &mut Vec<u8> = &mut msg;
	    stream.read_to_end(msg_ptr);
    }
    println!("{}", String::from_utf8(msg).unwrap());
  //   out_stream.write(String::from_utf8(msg).unwrap().as_bytes());
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