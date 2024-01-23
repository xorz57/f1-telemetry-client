pub mod packets;

use packets::packet_car_telemetry_data::PacketCarTelemetryData;
use packets::packet_header::PacketHeader;
use std::net::UdpSocket;

pub struct F1TelemetryClient {
    socket: UdpSocket,
    buffer: [u8; 2048],
}

impl F1TelemetryClient {
    pub fn new(bind_address: &str) -> Self {
        let socket: UdpSocket = UdpSocket::bind(bind_address).expect("Couldn't bind to address");
        let buffer: [u8; 2048] = [0; 2048];
        F1TelemetryClient { socket, buffer }
    }

    pub fn run(&mut self) {
        loop {
            self.receive_packet();
        }
    }

    fn receive_packet(&mut self) {
        match self.socket.recv(&mut self.buffer) {
            Ok(received) => {
                match PacketHeader::from_bytes(&self.buffer[..received]) {
                    Ok(header) => match header.packet_id {
                        6 => {
                            match PacketCarTelemetryData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => println!("{:?}", packet.car_telemetry_data[0]),
                                Err(e) => {
                                    eprintln!("{e:?}");
                                    return;
                                }
                            };
                        }
                        _ => {}
                    },
                    Err(e) => {
                        eprintln!("{e:?}");
                        return;
                    }
                };
            }
            Err(e) => {
                eprintln!("{e:?}");
                return;
            }
        }
    }
}
