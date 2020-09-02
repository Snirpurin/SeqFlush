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
 
    let mut handle_sender = vec![];
    for mut fs in filesenders{
        handle_sender.push(thread::spawn(move || {
            let mut file = fs;
            loop{
                if file.test(){
                    file.read_into_mem();
                }
                file.prep_packet();
                file.send(54);
                if file.done(){
                    println!("current is {} end is {}", file.get_current(),file.get_end());
                    println!("done");
                    break;
                }

            }
        }));
    } 
/*
    let mut handle_rec = vec![];

    let mut test_rec = File::create("test_rec_1gb").expect("cant create rec file");
   
    
    for socket in socket_client{
        let file = File::open("test_rec_1gb").expect("cant open file");
        handle_rec.push(thread::spawn(move || {
            let mut file = file;
            let mut rec_buf =[0;508];
            let mut bytes: u64 = 0;
            loop{
                let (data,s) = socket.recv_from(&mut rec_buf).expect("failed1");               
                file.write_all(&rec_buf).expect("failed2");
            }
        }));

    }
    for handle in handle_rec{
        handle.join();
    }
    */
    for handle in handle_sender{
        handle.join();
    }




    





}
