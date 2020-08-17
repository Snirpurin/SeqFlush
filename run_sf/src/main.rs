use seq_flush::*;
use std::net::UdpSocket;

fn main() {
    println!("Hello, world!");
    let socket_server = seq_flush::server::init_bind("77777");


    let socket_client = seq_flush::client::init_rec(8000, 8009);

    seq_flush::server::make_address([], "127.0.0.1::");

    

}
