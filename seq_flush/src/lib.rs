use std::net::UdpSocket;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::thread;

pub mod server{
    use super::*;

    pub fn init_bind(port:&str)->UdpSocket{
        
        let socket = UdpSocket::bind(format!("{}{}","127.0.0.1::", port)).unwrap();
        return socket;
    }

    
    // Splits the file into several file handlers
    pub fn init_file(path_file:String, num_seq:i32, seq_number:f32)->(Vec<File>,u64,u64){
        
        let file = File::open(&path_file).unwrap();
        let metadata = file.metadata().unwrap();
        let mut filehandles:Vec<File> = Vec::new();
        
        let size = metadata.len();
        let seq = (size as f32) / seq_number;
        let temp = seq % 1 as f32;
        let seq = ((1 as f32 - temp) + seq) as u64;
        let delta = (seq_number as u64 * seq) - size;
        let seq_last = seq - delta;

        for n in 0..=seq_number as u64 {
            let mut file = File::open(&path_file).unwrap();
            file.seek(SeekFrom::Start(n*seq)).unwrap();
            filehandles.push(file)
        }
        //returns the file handlers and the size of each seq and of the last one
        (filehandles,seq,seq_last)
        
    }

    pub fn init_file_to_socket(files: &Vec<File>) -> Vec<UdpSocket>{
        //
        let mut sockets: Vec<UdpSocket> = Vec::new();
        for file in files{
            sockets.push(UdpSocket::bind("0.0.0.0:0").unwrap());
        }
        sockets
    }

    pub fn make_address(range:&[u64], addr: &str)->Vec<String>{
        let mut address:Vec<String> = Vec::new();
        for port in range.iter(){
            address.push(format!("{}{}",addr, port));
        }
        address
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
    use super::*;

    pub fn init_rec(start:u64, end:u64)->Vec<UdpSocket>{
        let mut address:Vec<UdpSocket> = Vec::new();
        for port in start..=end{
            address.push(UdpSocket::bind(format!("{}{}","127.0.0.1::", port)).unwrap());
        }
        address
    }




}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
