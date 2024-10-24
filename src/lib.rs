pub mod packets;

use packets::PacketCarDamageData;
use packets::PacketCarSetupData;
use packets::PacketCarStatusData;
use packets::PacketCarTelemetryData;
use packets::PacketEventData;
use packets::PacketFinalClassificationData;
use packets::PacketHeader;
use packets::PacketLapData;
use packets::PacketLobbyInfoData;
use packets::PacketMotionData;
use packets::PacketMotionExData;
use packets::PacketParticipantsData;
use packets::PacketSessionData;
use packets::PacketSessionHistoryData;
use packets::PacketTyreSetsData;
use std::net::UdpSocket;

pub struct F1TelemetryClient {
    socket: UdpSocket,
    buf: [u8; 2048],
    car_damage_data_handler: Box<dyn Fn(&PacketCarDamageData)>,
    car_setup_data_handler: Box<dyn Fn(&PacketCarSetupData)>,
    car_status_data_handler: Box<dyn Fn(&PacketCarStatusData)>,
    car_telemetry_data_handler: Box<dyn Fn(&PacketCarTelemetryData)>,
    event_data_handler: Box<dyn Fn(&PacketEventData)>,
    final_classification_data_handler: Box<dyn Fn(&PacketFinalClassificationData)>,
    lap_data_handler: Box<dyn Fn(&PacketLapData)>,
    lobby_info_data_handler: Box<dyn Fn(&PacketLobbyInfoData)>,
    motion_data_handler: Box<dyn Fn(&PacketMotionData)>,
    motion_ex_data_handler: Box<dyn Fn(&PacketMotionExData)>,
    participants_data_handler: Box<dyn Fn(&PacketParticipantsData)>,
    session_data_handler: Box<dyn Fn(&PacketSessionData)>,
    session_history_data_handler: Box<dyn Fn(&PacketSessionHistoryData)>,
    tyre_sets_data_handler: Box<dyn Fn(&PacketTyreSetsData)>,
}

impl F1TelemetryClient {
    pub fn new(bind_address: &str) -> Self {
        let socket: UdpSocket = UdpSocket::bind(bind_address).expect("Couldn't bind to address");
        let buf: [u8; 2048] = [0; 2048];
        let car_damage_data_handler = Box::new(|_: &PacketCarDamageData| {});
        let car_setup_data_handler = Box::new(|_: &PacketCarSetupData| {});
        let car_status_data_handler = Box::new(|_: &PacketCarStatusData| {});
        let car_telemetry_data_handler = Box::new(|_: &PacketCarTelemetryData| {});
        let event_data_handler = Box::new(|_: &PacketEventData| {});
        let final_classification_data_handler = Box::new(|_: &PacketFinalClassificationData| {});
        let lap_data_handler = Box::new(|_: &PacketLapData| {});
        let lobby_info_data_handler = Box::new(|_: &PacketLobbyInfoData| {});
        let motion_data_handler = Box::new(|_: &PacketMotionData| {});
        let motion_ex_data_handler = Box::new(|_: &PacketMotionExData| {});
        let participants_data_handler = Box::new(|_: &PacketParticipantsData| {});
        let session_data_handler = Box::new(|_: &PacketSessionData| {});
        let session_history_data_handler = Box::new(|_: &PacketSessionHistoryData| {});
        let tyre_sets_data_handler = Box::new(|_: &PacketTyreSetsData| {});

        F1TelemetryClient {
            socket,
            buf,
            car_damage_data_handler,
            car_setup_data_handler,
            car_status_data_handler,
            car_telemetry_data_handler,
            event_data_handler,
            final_classification_data_handler,
            lap_data_handler,
            lobby_info_data_handler,
            motion_data_handler,
            motion_ex_data_handler,
            participants_data_handler,
            session_data_handler,
            session_history_data_handler,
            tyre_sets_data_handler,
        }
    }

    pub fn set_packet_car_damage_data_handler(&mut self, handler: Box<dyn Fn(&PacketCarDamageData)>) {
        self.car_damage_data_handler = handler;
    }

    pub fn set_packet_car_setup_data_handler(&mut self, handler: Box<dyn Fn(&PacketCarSetupData)>) {
        self.car_setup_data_handler = handler;
    }

    pub fn set_packet_car_status_data_handler(&mut self, handler: Box<dyn Fn(&PacketCarStatusData)>) {
        self.car_status_data_handler = handler;
    }

    pub fn set_packet_car_telemetry_data_handler(&mut self, handler: Box<dyn Fn(&PacketCarTelemetryData)>) {
        self.car_telemetry_data_handler = handler;
    }

    pub fn set_packet_event_data_handler(&mut self, handler: Box<dyn Fn(&PacketEventData)>) {
        self.event_data_handler = handler;
    }

    pub fn set_packet_final_classification_data_handler(
        &mut self,
        handler: Box<dyn Fn(&PacketFinalClassificationData)>,
    ) {
        self.final_classification_data_handler = handler;
    }

    pub fn set_packet_lap_data_handler(&mut self, handler: Box<dyn Fn(&PacketLapData)>) {
        self.lap_data_handler = handler;
    }

    pub fn set_packet_lobby_info_data_handler(&mut self, handler: Box<dyn Fn(&PacketLobbyInfoData)>) {
        self.lobby_info_data_handler = handler;
    }

    pub fn set_packet_motion_data_handler(&mut self, handler: Box<dyn Fn(&PacketMotionData)>) {
        self.motion_data_handler = handler;
    }

    pub fn set_packet_motion_ex_data_handler(&mut self, handler: Box<dyn Fn(&PacketMotionExData)>) {
        self.motion_ex_data_handler = handler;
    }

    pub fn set_packet_participants_data_handler(&mut self, handler: Box<dyn Fn(&PacketParticipantsData)>) {
        self.participants_data_handler = handler;
    }

    pub fn set_packet_session_data_handler(&mut self, handler: Box<dyn Fn(&PacketSessionData)>) {
        self.session_data_handler = handler;
    }

    pub fn set_packet_session_history_data_handler(&mut self, handler: Box<dyn Fn(&PacketSessionHistoryData)>) {
        self.session_history_data_handler = handler;
    }

    pub fn set_packet_tyre_sets_data_handler(&mut self, handler: Box<dyn Fn(&PacketTyreSetsData)>) {
        self.tyre_sets_data_handler = handler;
    }

    pub fn run(&mut self) {
        loop {
            self.receive_packet();
        }
    }

    fn receive_packet(&mut self) {
        match self.socket.recv(&mut self.buf) {
            Ok(received) => {
                match PacketHeader::unserialize(&self.buf[..received]) {
                    Ok(header) => match header.packet_id {
                        0 => {
                            match PacketMotionData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.motion_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        1 => {
                            match PacketSessionData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.session_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        2 => {
                            match PacketLapData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.lap_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        3 => {
                            match PacketEventData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.event_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        4 => {
                            match PacketParticipantsData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.participants_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        5 => {
                            match PacketCarSetupData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.car_setup_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        6 => {
                            match PacketCarTelemetryData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.car_telemetry_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        7 => {
                            match PacketCarStatusData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.car_status_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        8 => {
                            match PacketFinalClassificationData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.final_classification_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        9 => {
                            match PacketLobbyInfoData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.lobby_info_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        10 => {
                            match PacketCarDamageData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.car_damage_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        11 => {
                            match PacketSessionHistoryData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.session_history_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        12 => {
                            match PacketTyreSetsData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.tyre_sets_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        13 => {
                            match PacketMotionExData::unserialize(&self.buf[..received]) {
                                Ok(packet) => (self.motion_ex_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        _ => {}
                    },
                    Err(e) => eprintln!("{e:?}"),
                };
            }
            Err(e) => eprintln!("{e:?}"),
        }
    }
}
