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



    let mut filesenders = seq_flush::server::init_full("test.txt", 10, "127.0.0.1", 8000, 8009);

    for mut fs in filesenders{
        fs.read_into_mem();
        fs.prep_packet();
        fs.send(54);
    }


   let handle = thread::spawn(move || {
        let mut test_rec = File::create("test_rec.txt").unwrap();
        let mut rec_buf:Vec<u8> =vec![0;10];
        for socket in socket_client{
            socket.recv_from(&mut rec_buf).expect("failed1");
            test_rec.write_all(&rec_buf).expect("failed2");
        }
        let meta =test_rec.metadata().unwrap();
        println!("{}",meta.len());
    
        println!("finished");

    });
    handle.join().unwrap();



}
