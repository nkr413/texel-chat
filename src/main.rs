// packages
// use std::thread;
// use std::net::{TcpListener, TcpStream, Shutdown};
// use std::io::{Write, Read};
// use std::str::from_utf8;

// modules
mod client;
pub use crate::client::client_mod;

mod server;
pub use crate::server::server_mod;

fn main() {
	let action = std::env::args().nth(1).expect("no pattern given");

	if action == "server" { server_mod::server_fn(); }
	else if action == "client" { client_mod::client_fn(); }
	else {
		println!("Incorrect command !");
		return;
	}
}