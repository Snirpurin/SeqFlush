use seq_flush::*;
use std::net::UdpSocket;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::thread;

fn main() {
    println!("Hello, world!");
    //let socket_server = seq_flush::server::init_bind("77777");
    let data: [u8;1000] = [1;1000];
    let mut test_file = File::create("../../test.txt").unwrap();
    test_file.write_all(&mut data);
    let socket_client = seq_flush::client::init_rec(8000, 8009);

    let addresses = seq_flush::server::make_address(8000, 8009, "127.0.0.1");

    let (mut files,size,size_last) = seq_flush::server::init_file("../../test.txt", 10.0);
    let socket_server = seq_flush::server::init_file_to_socket(&files);
    let socket_server = seq_flush::server::connect(addresses, socket_server);

    for (file, socket) in files.iter().zip(socket_server){
        seq_flush::server::sender(file, size, socket);
    }

    

}
