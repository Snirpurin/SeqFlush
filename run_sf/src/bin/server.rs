use seq_flush::*;
use std::net::UdpSocket;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::thread;
use std::env;
use std::time::{Duration, Instant};
use std::io::ErrorKind;

enum State{
    UncWait,
    CWait,
    
}
fn main(){
    
    //let list = env::args().nth(1).unwrap();
    //let arg_pr = env::args().nth(2).unwrap();
    
    let mut sock = seq_flush::server::init_bind("8888");


    let start = Instant::now();
    let mut buf = [0u8; 1024];

    let mut prot = [0u8;4];
    let mut file_name = [0u8;500];
    while start.elapsed().as_secs() < 5 {
        let result = sock.recv(&mut buf);
        match result {
            Ok(num_bytes) => println!("I received {} bytes!", num_bytes),
            Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                println!("Something went wrong: {}", err)
            }
            _ => {}
        }
        
        prot.copy_from_slice(&mut buf[..4]);
        let match_prot = unsafe {
            std::mem::transmute::<[u8; 4],u32>(prot)
        };
        match match_prot {
            _ =>{},
        }
        thread::sleep(Duration::from_millis(5));
    }




}