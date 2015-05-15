// Client Demo

use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::io;
// use std::io::net::tcp::TcpStream;

fn main() {
	
	let mut stream = TcpStream::connect("127.0.0.1:4000").unwrap();

	// let mut buf1 = "hello";
	// let _ = stream.write(b"hello1");
	
	// let mut buf: &mut [u8];
	// let mut res: Vec<u8> = Vec::new();
 //    {
	//     let mut res_ptr: &mut Vec<u8> = &mut res;
	//     let _ = stream.read_to_end(res_ptr);
 //    }
 //    println!("{}", String::from_utf8(res).unwrap());
	
	let mut buf;
	let mut bufv: Vec<u8>;
    loop {
        // clear out the buffer so we don't send garbage
        buf = [0; 512];
        bufv = Vec::new();
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF
                    break;
                }
                bufv.write_all(&buf);
                println!("{}", String::from_utf8(bufv).unwrap());
                m
            },
        };

        match stream.write(&buf) {
            Err(_) => break,
            Ok(_) => {
            	println!("Wrote something");
            	// bufv.write_all(&buf);
             //    println!("{}", String::from_utf8(bufv).unwrap());
            	// continue,
            }
        }
    }
	
	drop(stream); // close the connection
}