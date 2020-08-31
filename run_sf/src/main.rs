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



    let mut filesenders = seq_flush::server::init_full("test_1gb", 10, "127.0.0.1", 8000, 8009);
    let handle1=  thread::spawn(move || {
        let mut holder = filesenders;
        loop {
            for mut fs in &mut holder{
                fs.read_into_mem();
                fs.prep_packet();
                fs.send(54);
                fs.prep_packet();
                fs.send(54);
                if fs.get_current() > fs.get_end() - 501{
                    break;
                }
            }
        }

    }); 


   let handle = thread::spawn(move || {
        let mut test_rec = File::create("test_rec_1gb").unwrap();
        let mut rec_buf =[0;508];
        let mut socket_client = socket_client;
        let meta =test_rec.metadata().unwrap();
        let mut bytes: u64 = 0;
        loop{
            for socket in &mut socket_client{
                let (data,s) = socket.recv_from(&mut rec_buf).expect("failed1");
                bytes = bytes + data as u64;
                test_rec.write_all(&rec_buf).expect("failed2");
            }
            if bytes> 800000000{
                break;
            }
        }

        
        println!("{}",meta.len());
    
        println!("finished");

    });
    handle.join().unwrap();
    handle1.join().unwrap();



}
