use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::net::UdpSocket;
use std::thread;

const FILE_BUF_SIZE: usize = 1000;


const HEADER_PROTOCOL_SIZE: usize = 4;
const HEADER_PACKET_SIZE: usize = 4;
const HEADER_SIZE: usize = HEADER_PACKET_SIZE + HEADER_PROTOCOL_SIZE;
const DATA_SIZE: usize = 500;
const PACKET_BUF_SIZE: usize = DATA_SIZE + HEADER_SIZE;

pub mod server {
    use super::*;

    pub struct FileSender<F>{
        file:F,
        current:u64,
        start:u64,
        end:u64,
        size:u64,
        index:u64,
        socket:UdpSocket,
        buffer:Vec<u8>,
        buf_index:u64,
        buf_index_end:u64,
        packet:Packet,
    }

    pub struct Packet{
        data:[u8;PACKET_BUF_SIZE],
        current_size:u32,
        header_packet_size:u32,
        header_protocol:u32,
    }

    impl<F: Read + Seek> FileSender<F>{
        pub fn new( file:F, start:u64, end:u64, index:u64, socket:UdpSocket)->Self{
                FileSender{
                    file,
                    current:start,
                    start:start,
                    end:end,
                    size:end-start,
                    index:index,
                    socket:socket,
                    //buffer:vec![2u8;FILE_BUF_SIZE],
                    buffer:Vec::with_capacity(FILE_BUF_SIZE),
                    buf_index:0,
                    buf_index_end:FILE_BUF_SIZE as u64,
                    packet:Packet{data:[0u8;PACKET_BUF_SIZE],
                        current_size:0,
                        header_packet_size:0,
                        header_protocol:0},
                }
        }
        
        pub fn get_current(&mut self)->u64{
            self.current
        }

        pub fn get_end(&mut self)->u64{
            self.end
        }


        pub fn read_into_mem(&mut self) ->Result<(),io::ErrorKind>{
            let mut size = FILE_BUF_SIZE as u64;
            if self.current + FILE_BUF_SIZE as u64 > self.end{
                size = self.end - (FILE_BUF_SIZE as u64 +self.current);
            }
            let mut chunk = self.file.by_ref().take(size);
            let n = chunk.read_to_end(&mut self.buffer).expect("Did not read enough");
            self.current = self.current + size;
            Ok(())
        }

        pub fn prep_packet(&mut self){
            let packetsize = self.packet.header_packet_size;
            let protocol = self.packet.header_protocol;
            let size = unsafe {
                std::mem::transmute::<u32,[u8; 4]>(packetsize)
            };
            let protocol = unsafe {
                std::mem::transmute::<u32,[u8; 4]>(protocol)
            };

            self.packet.data[HEADER_SIZE..].copy_from_slice(&mut self.buffer[(self.buf_index as usize)..(self.buf_index + (DATA_SIZE) as u64) as usize]);
            self.packet.data[..HEADER_PACKET_SIZE].copy_from_slice(&size);
            self.packet.data[HEADER_PACKET_SIZE..(HEADER_PACKET_SIZE+HEADER_PROTOCOL_SIZE)].copy_from_slice(&protocol);
            //println!("{:?}",self.packet.data);
        }

        pub fn send(&mut self, size:u64) {
            
            self.socket.send(&mut self.packet.data);

            
        }



    }



    pub fn init_bind(port: &str) -> UdpSocket {
        let socket = UdpSocket::bind(format!("{}{}", "127.0.0.1::", port)).unwrap();
        return socket;
    }

    // Splits the file into several file handlers
    pub fn init_filesender(path_file: &str, seq_number: u64, sockets:Vec<UdpSocket>) -> Vec<FileSender<File>> {
        let mut filesender:Vec<FileSender<File>> = Vec::new();
        let file = File::open(&path_file).unwrap();
        let metadata = file.metadata().unwrap();
        let mut filehandles: Vec<File> = Vec::new();
        
        let size = metadata.len();
        let seq = (size as f32) / seq_number as f32;
        let temp = seq % 1 as f32;
        let seq = ((1 as f32 - temp) + seq) as u64;
        let delta = (seq_number as u64 * seq) - size;
        let seq_last = seq - delta;

        for (n,socket) in (0..seq_number).zip(sockets) {
            let mut file = File::open(&path_file).unwrap();
            file.seek(SeekFrom::Start(n * seq)).unwrap();
            if n == seq_number as u64{
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
