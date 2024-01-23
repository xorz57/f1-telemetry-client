pub mod packet_car_damage_data;
pub mod packet_car_setup_data;
pub mod packet_car_status_data;
pub mod packet_car_telemetry_data;
pub mod packet_event_data;
pub mod packet_final_classification_data;
pub mod packet_header;
pub mod packet_lap_data;
pub mod packet_lobby_info_data;
pub mod packet_motion_data;
pub mod packet_motion_ex_data;
pub mod packet_participants_data;
pub mod packet_session_data;
pub mod packet_session_history_data;
pub mod packet_tyre_sets_data;

use packet_car_telemetry_data::PacketCarTelemetryData;
use packet_header::PacketHeader;
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
