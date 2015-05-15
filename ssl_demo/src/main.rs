use std::net::tcp;
use std::net::ip;
use std::task;
use std::uv;
extern crate pipes;
use pipes::{stream, Port, Chan};
 
type ConnectMsg = (tcp::TcpNewConnection, core::oldcomm::Chan<Option<tcp::TcpErrData>>);

fn main() {
	//Connection information will be transmitted using this Port and Chan
	let (port, chan): (Port<ConnectMsg>, Chan<ConnectMsg>) = stream();
	
	//A separate task handles connections
	spawn( {
		loop {
			let (conn, kill_ch) = port.recv();
			log(info, "a new connection");
			match tcp::accept(conn) {
				result::Err(err) => {
					log(error, "Connection error");
					kill_ch.send(Some(err));
				},
				result::Ok(socket) => {
					log(info, "Connection accepted");
					let peer_addr = ip::format_addr(&socket.get_peer_addr());
					
					let socket_buf = tcp::socket_buf(socket);
					let socket_read = (socket_buf as io::ReaderUtil);
					let socket_write = (socket_buf as io::WriterUtil);
					
					socket_write.write_str("Hello from server\n");
					println!("{}> {}", peer_addr, socket_read.read_line());
					socket_write.write_str("Goodbye from server\n");
				}
			}
		}
	});

	//Listen for incomming connections
	tcp::listen(
		ip::v4::parse_addr("127.0.0.1"),
		7777,
		5,
		uv::global_loop::get(),
		|_| {
			//This callback passes the kill channel
			log(info, "server is listening")
		},
		|conn, kill_ch| {
			/* This callback executes when a connection is received
			 * The connection must be accepted from another task or
			 * the server will block.
			 */
			chan.send((conn, kill_ch));
		}
	);
}