use std::net::tcp;
use std::net::ip;
use std::uv;

fn main() {
	match tcp::connect(ip::v4::parse_addr("127.0.0.1"), 7777, uv::global_loop::get()) {
		//Handle a connection error
		result::Err(err) => match err {
			tcp::GenericConnectErr(name,msg) => log(error, fmt!("Connection error %s: %s", name, msg)),
			tcp::ConnectionRefused => log(error, "Connection refused")
		},
		result::Ok(socket) => {
			let peer_addr: ~str = ip::format_addr(&socket.get_peer_addr());
			let socket_buf = tcp::socket_buf(socket);
			let socket_read = (socket_buf as io::ReaderUtil);
			let socket_write = (socket_buf as io::WriterUtil);
						
			io::println(fmt!("%s> %s", peer_addr, socket_read.read_line()));
			socket_write.write_str("Hello from client\n");
			io::println(fmt!("%s> %s", peer_addr, socket_read.read_line()));
		}		
	}
}