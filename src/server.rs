use std::io::{Error, Write};
use std::io::ErrorKind::InvalidData;
use std::net::UdpSocket;
use std::thread;
use std::thread::{JoinHandle, Thread};
use crate::AppInfo;
use crate::handshake::{HsOp, HsSerializable, HsStatus};
use crate::osc::OscMessage;

pub(crate) struct Server<'a> {
    info: &'a AppInfo<'a>,
    pub(crate) commander_thread: JoinHandle<()>,
    client_streams: Vec<UdpSocket>,
    recv_stream: UdpSocket,
}

impl Server<'_> {
    pub fn new<'a>(info: &'static AppInfo, bind_addr: &str, listen_port: i32, handshake_port: i32) -> std::io::Result<Server<'a>> {
        let commander_stream = UdpSocket::bind(format!("{}:{}", bind_addr, handshake_port))?;
        println!("Listening for handshake on port {}", handshake_port);
        let thread = thread::spawn(move || loop {
            match Server::handle_commander(&commander_stream) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error handling handshake: {}", e);
                    break;
                }
            }
        } );

        let recv_stream = UdpSocket::bind(format!("{}:{}", bind_addr, listen_port))?;
        println!("Listening for packets on port {}", listen_port);
        Ok(Server {
            info,
            commander_thread: thread,
            client_streams: Vec::new(),
            recv_stream,
        })
    }

    fn handle_commander(handshake_stream: &UdpSocket) -> std::io::Result<()> {
        println!("Waiting for handshake...");
        let mut data = [0; 1024];
        match handshake_stream.recv_from(&mut data) {
            Ok((size, addr)) => {
                // Cut the data to the size of the packet
                let mut data = data.to_vec();
                data.truncate(size);
                // Try parse this to a handshake packet
                match OscMessage::new(&data) {
                    Ok(msg) => {
                        println!("Received osc message at address {} from {}", msg.address, addr);
                        Ok(())
                    }
                    Err(e) => {
                        println!("{}", e);
                        Err(Error::new(InvalidData, "Invalid handshake packet"))
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                Err(e)
            }
        }
    }
}