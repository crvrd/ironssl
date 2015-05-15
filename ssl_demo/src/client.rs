// Client Demo

use std::io::TcpStream;
use std::io::prelude::*;

fn main() {

	let mut stream = TcpStream::connect("127.0.0.1:5000");

	stream.write("hi");
	
	let mut buf = [0];
	stream.read(buf);
	println!("{:s}", buf);
	
	drop(stream); // close the connection
}