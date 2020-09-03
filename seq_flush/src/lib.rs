use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::net::UdpSocket;
use std::thread;

const FILE_BUF_SIZE: usize = 10000;


const HEADER_PROTOCOL_SIZE: usize = 4;
const HEADER_PACKET_SIZE: usize = 4;
const HEADER_SIZE: usize = HEADER_PACKET_SIZE + HEADER_PROTOCOL_SIZE;
const DATA_SIZE: usize = 500;
const PACKET_BUF_SIZE: usize = DATA_SIZE + HEADER_SIZE;

pub mod server {
    use super::*;

    pub struct FileSender<F>{
        file:F,
        current:usize,
        start:usize,
        end:usize,
        size:usize,
        index:usize,
        socket:UdpSocket,
        buffer:Vec<u8>,
        buf_index:usize,
        buf_chunks:usize,
        packet:Packet,
    }

    pub struct Packet{
        data:[u8;PACKET_BUF_SIZE],
        current_size:u32,
        header_packet_size:u32,
        header_protocol:u32,
    }

    impl<F: Read + Seek> FileSender<F>{
        pub fn new( file:F, start:usize, end:usize, index:usize, socket:UdpSocket)->Self{
                FileSender{
                    file,
                    current:start,
                    start:start,
                    end:end,
                    size:end-start,
                    index:index,
                    socket:socket,

                    buffer:Vec::with_capacity(FILE_BUF_SIZE),
                    buf_index:0,
                    buf_chunks: 0,

                    packet:Packet{data:[0u8;PACKET_BUF_SIZE],
                        current_size:0,
                        header_packet_size:0,
                        header_protocol:0},
                }
        }
        
        pub fn get_current(&mut self)->usize{
            self.current
        }

        pub fn get_end(&mut self)->usize{
            self.end
        }

        pub fn test(&mut self)-> bool{
            return self.buf_index >= self.buffer.len()
        }
        pub fn done(&mut self)->bool{
            self.current == self.end && self.buf_index >= self.buffer.len()
        }

        pub fn read_into_mem(&mut self) ->Result<(),io::ErrorKind>{
            self.buf_index = 0;
            self.buffer.clear();
            let mut size = FILE_BUF_SIZE;
            if self.current + FILE_BUF_SIZE > self.end{
                size = self.end - (FILE_BUF_SIZE + self.current);
            }
            let mut chunk = self.file.by_ref().take(size as u64);
            let n = chunk.read_to_end(&mut self.buffer).expect("Did not read enough");
            //println!("{}",n);
            self.current = self.current + size;
            self.buf_chunks = size / DATA_SIZE;
            Ok(())
        }

        //possible protocol her as input
        pub fn prep_packet(&mut self){


            let mut packetsize = DATA_SIZE;
            let protocol = self.packet.header_protocol;
            if (self.buf_index + DATA_SIZE) > self.buffer.len() && self.buf_index !=self.buffer.len(){
                packetsize = self.buffer.len() - self.buf_index;
                
            }
            //println!("{}",packetsize);
            let size = unsafe {
                std::mem::transmute::<u32,[u8; 4]>(packetsize as u32)
            };
            let protocol = unsafe {
                std::mem::transmute::<u32,[u8; 4]>(protocol)
            };


            self.packet.data[HEADER_SIZE..].copy_from_slice(&mut self.buffer[(self.buf_index)..(self.buf_index + packetsize)]);
            self.buf_index = self.buf_index + packetsize;
            self.packet.data[..HEADER_PACKET_SIZE].copy_from_slice(&size);
            self.packet.data[HEADER_PACKET_SIZE..(HEADER_PACKET_SIZE+HEADER_PROTOCOL_SIZE)].copy_from_slice(&protocol);
            //println!("{:?}",self.packet.data);
        }

        pub fn send(&mut self, size:u64) {
            
            self.socket.send(&mut self.packet.data);

            
        }



    }



    pub fn init_bind(port: &str) -> UdpSocket {
        let sock = UdpSocket::bind("0.0.0.0:8888").expect("Failed to bind socket");
        sock.set_nonblocking(true)
            .expect("Failed to enter non-blocking mode");

        return sock;
    }

    // Splits the file into several file handlers
    pub fn init_filesender(path_file: &str, seq_number: usize, sockets:Vec<UdpSocket>) -> Vec<FileSender<File>> {
        let mut filesender:Vec<FileSender<File>> = Vec::new();
        let file = File::open(&path_file).unwrap();
        let metadata = file.metadata().unwrap();
        let mut filehandles: Vec<File> = Vec::new();
        
        let size = metadata.len() as usize;
        let seq = (size as f32) / seq_number as f32;
        let temp = seq % 1 as f32;
        let seq = ((1 as f32 - temp) + seq) as usize;
        let delta = (seq_number as usize * seq) - size;
        let seq_last = seq - delta;

        for (n,socket) in (0..seq_number).zip(sockets) {
            let mut file = File::open(&path_file).unwrap();
            file.seek(SeekFrom::Start((n * seq) as u64)).unwrap();
            if n == seq_number{
                filesender.push(FileSender::new(file, n * seq, n * seq + seq_last, n, socket));
            }
            else{
                filesender.push(FileSender::new(file, n * seq, n * seq + seq, n, socket));
            }
            
        }
        //returns the file handlers and the size of each seq and of the last one
        filesender
    }

    pub fn init_full(path: &str, n_seq: u64, addres: &str, port_st: u64, port_end: u64) -> Vec<FileSender<File>> {
        let addresses = make_address(port_st, port_end, addres);
        let socket_server = init_socket(addresses);
        init_filesender(path, 10, socket_server)
        
    }


    pub fn init_socket(address: Vec<String>) -> Vec<UdpSocket> {
        //
        let mut sockets: Vec<UdpSocket> = Vec::new();
        for adres in address {
            let mut so = UdpSocket::bind("0.0.0.0:0").unwrap();
            so.connect(adres);
            sockets.push(so);
        }
        sockets
    }

    pub fn make_address(start: u64, end: u64, addr: &str) -> Vec<String> {
        let mut address: Vec<String> = Vec::new();
        for port in start..=end {
            address.push(format!("{}:{}", addr, port));
        }
        address
    }

 

}

pub mod client {
    use super::*;

    pub fn init_rec(start: u64, end: u64) -> Vec<UdpSocket> {
        let mut address: Vec<UdpSocket> = Vec::new();
        for port in start..=end {
            address.push(UdpSocket::bind(format!("{}:{}", "127.0.0.1", port)).unwrap());
        }
        address
    }

    pub fn init_file(path_file: &str, seq_number: f32, size:u64) -> (Vec<File>, u64, u64) {
        let file = File::create(&path_file).unwrap();
        let metadata = file.metadata().unwrap();
        let mut filehandles: Vec<File> = Vec::new();
        let size = metadata.len();
        let seq = (size as f32) / seq_number;
        let temp = seq % 1 as f32;
        let seq = ((1 as f32 - temp) + seq) as u64;
        let delta = (seq_number as u64 * seq) - size;
        let seq_last = seq - delta;

        for n in 0..=seq_number as u64 {
            let mut file = File::open(&path_file).unwrap();
            file.seek(SeekFrom::Start(n * seq)).unwrap();
            filehandles.push(file)
        }
        //returns the file handlers and the size of each seq and of the last one
        (filehandles, seq, seq_last)
    }




}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
