// Client Demo

use std::net::TcpStream;
use std::io::Write;
// use std::io::net::tcp::TcpStream;

fn main() {
	
	let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();

	// let mut buf1 = "hello";
	stream.write(b"hello");
	
	// let mut buf = "hi";
	// stream.read(&buf);
	// println!("{:?}", buf);
	
	drop(stream); // close the connection
}