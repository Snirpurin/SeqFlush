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
    //let mut data: [u8;1000] = [1;1000];
    //let mut test_file = File::create("test.txt").unwrap();
    //test_file.write_all(&mut data).unwrap();
    let socket_client = seq_flush::client::init_rec(8000, 8009);

    let addresses = seq_flush::server::make_address(8000, 8009, "127.0.0.1");

    let (mut files,size,size_last) = seq_flush::server::init_file("test.txt", 10.0);
    let socket_server = seq_flush::server::init_socket(&files);
    let socket_server = seq_flush::server::connect(addresses, socket_server);
    println!("size is : {}, end size is : {}", size,size_last);

    for (file, socket) in files.iter().zip(socket_server){
        seq_flush::server::sender(file, size, socket);
    }



    let mut test_rec = File::create("test_rec.txt").unwrap();
    let mut rec_buf:Vec<u8> =vec![0;112];
    for socket in socket_client{
        socket.recv_from(&mut rec_buf).expect("failed1");
        test_rec.write_all(&rec_buf).expect("failed2");
    }
    let meta =test_rec.metadata().unwrap();
    println!("{}",meta.len());

    println!("finished");
}
