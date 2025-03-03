use std::net::UdpSocket;

use crate::{models::packet::NtpPacket, resolver::NtpResolver};

pub struct NtpServer {
    port: u32,
    address: String
}

impl NtpServer {
    pub fn init(address: String, port: u32) -> Self {
        Self { port, address }
    }

    pub fn start(&self) {
        let socket = UdpSocket::bind(format!("{}:{}", self.address, self.port)).unwrap();

        println!("Listening on port {}", self.port);

        loop {
            let mut buff: [u8; 1024] = [0; 1024];
            
            let (_, addr) = socket.recv_from(&mut buff).unwrap();

            println!("Got request from {}", addr);

            let packet = NtpPacket::from_bytes(&buff).unwrap();

            println!("{:?}", packet);
            let resolved = NtpResolver::resolve(&packet);
            
            match socket.send_to(&resolved.to_bytes(), addr) {
                Ok(_) => {
                    println!("Successfully sent the packet back");
                }

                Err(_) => {
                    println!("Failed to send the packet back!");
                }
            }
        }
    }
}
