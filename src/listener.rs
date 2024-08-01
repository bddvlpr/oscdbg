use std::{
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{self, Receiver},
    thread,
};

use rosc::{decoder, OscPacket};

pub type Listener = Receiver<ListenerMessage>;

pub enum ListenerMessage {
    PacketReceived(OscPacket, SocketAddr),
    Error(String),
}

pub fn create_listener(addr: SocketAddr) -> Listener {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let socket = UdpSocket::bind(addr).expect("Failed to bind to the target socket");
        loop {
            let mut buf = [0u8; 1024];
            let (size, addr) = socket
                .recv_from(&mut buf)
                .expect("Failed to receive a buffer through the socket");
            match decoder::decode_udp(&buf[..size]) {
                Ok((_remainder, packet)) => {
                    tx.send(ListenerMessage::PacketReceived(packet, addr))
                        .unwrap();
                }
                Err(err) => {
                    tx.send(ListenerMessage::Error(format!(
                        "Failed decoding packet: {err}"
                    )))
                    .unwrap();
                }
            }
        }
    });

    rx
}
