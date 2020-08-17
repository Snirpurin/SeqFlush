use std::net::UdpSocket;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::thread;

pub mod server{
    use super::*;

    pub fn init_bind(port:String)->UdpSocket{
        
        let socket = UdpSocket::bind(format!("{}{}","127.0.0.1::", port)).unwrap();
        return socket;
    }

    
    // Splits the file into several file handlers
    pub fn init_file(path_file:String, num_seq:i32)->Vec<File>{
        
        let file = File::open(&path_file).unwrap();
        let metadata = file.metadata().unwrap();
        let mut filehandles:Vec<File> = Vec::new();
        //let size = metadata.len()/num_seq;
        for n in 0..=9 {
            let mut file = File::open(&path_file).unwrap();
            file.seek(SeekFrom::Start(n*100)).unwrap();
            filehandles.push(file)
        }
        filehandles
        
    }

    pub fn init_file_to_socket(files: &Vec<File>) -> Vec<UdpSocket>{
        //
        let mut sockets: Vec<UdpSocket> = Vec::new();
        for file in files{
            sockets.push(UdpSocket::bind("0.0.0.0:0").unwrap());
        }
        sockets
    }

    pub fn connect(addres:Vec<String>, mut sockets:Vec<UdpSocket>)->Vec<UdpSocket>{
        for (socket,addr) in sockets.iter().zip(addres){
            socket.connect(addr);
        }
        sockets
    } 
    
    pub fn sender(files:Vec<File>, sizes:u64, addres:Vec<u32>){




    }

    pub fn thread_send(file:File, adddr:String)->Result<(),()>{
        
        return Ok(());
    }
}

pub mod client {

    pub fn init_rec(){}


}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
