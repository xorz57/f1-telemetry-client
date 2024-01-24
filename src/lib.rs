pub mod packets;

use packets::PacketCarTelemetryData;
use packets::PacketHeader;
use std::net::UdpSocket;

pub struct F1TelemetryClient {
    socket: UdpSocket,
    buffer: [u8; 2048],
    car_telemetry_handler: Box<dyn Fn(&PacketCarTelemetryData)>,
}

impl F1TelemetryClient {
    pub fn new(bind_address: &str) -> Self {
        let socket: UdpSocket = UdpSocket::bind(bind_address).expect("Couldn't bind to address");
        let buffer: [u8; 2048] = [0; 2048];
        let car_telemetry_handler = Box::new(|_: &PacketCarTelemetryData| {});

        F1TelemetryClient {
            socket,
            buffer,
            car_telemetry_handler,
        }
    }

    pub fn set_car_telemetry_handler(&mut self, handler: Box<dyn Fn(&PacketCarTelemetryData)>) {
        self.car_telemetry_handler = handler;
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
                                Ok(packet) => (self.car_telemetry_handler)(&packet),
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
