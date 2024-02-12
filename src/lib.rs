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
    buffer: [u8; 2048],
    car_damage_handler: Box<dyn Fn(&PacketCarDamageData)>,
    car_setup_handler: Box<dyn Fn(&PacketCarSetupData)>,
    car_status_handler: Box<dyn Fn(&PacketCarStatusData)>,
    car_telemetry_handler: Box<dyn Fn(&PacketCarTelemetryData)>,
    event_handler: Box<dyn Fn(&PacketEventData)>,
    final_classification_handler: Box<dyn Fn(&PacketFinalClassificationData)>,
    lap_data_handler: Box<dyn Fn(&PacketLapData)>,
    lobby_info_handler: Box<dyn Fn(&PacketLobbyInfoData)>,
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
        let buffer: [u8; 2048] = [0; 2048];
        let car_damage_handler = Box::new(|_: &PacketCarDamageData| {});
        let car_setup_handler = Box::new(|_: &PacketCarSetupData| {});
        let car_status_handler = Box::new(|_: &PacketCarStatusData| {});
        let car_telemetry_handler = Box::new(|_: &PacketCarTelemetryData| {});
        let event_handler = Box::new(|_: &PacketEventData| {});
        let final_classification_handler = Box::new(|_: &PacketFinalClassificationData| {});
        let lap_data_handler = Box::new(|_: &PacketLapData| {});
        let lobby_info_handler = Box::new(|_: &PacketLobbyInfoData| {});
        let motion_data_handler = Box::new(|_: &PacketMotionData| {});
        let motion_ex_data_handler = Box::new(|_: &PacketMotionExData| {});
        let participants_data_handler = Box::new(|_: &PacketParticipantsData| {});
        let session_data_handler = Box::new(|_: &PacketSessionData| {});
        let session_history_data_handler = Box::new(|_: &PacketSessionHistoryData| {});
        let tyre_sets_data_handler = Box::new(|_: &PacketTyreSetsData| {});

        F1TelemetryClient {
            socket,
            buffer,
            car_damage_handler,
            car_setup_handler,
            car_status_handler,
            car_telemetry_handler,
            event_handler,
            final_classification_handler,
            lap_data_handler,
            lobby_info_handler,
            motion_data_handler,
            motion_ex_data_handler,
            participants_data_handler,
            session_data_handler,
            session_history_data_handler,
            tyre_sets_data_handler,
        }
    }

    pub fn on_car_damage(&mut self, handler: Box<dyn Fn(&PacketCarDamageData)>) {
        self.car_damage_handler = handler;
    }

    pub fn on_car_setup(&mut self, handler: Box<dyn Fn(&PacketCarSetupData)>) {
        self.car_setup_handler = handler;
    }

    pub fn on_car_status(&mut self, handler: Box<dyn Fn(&PacketCarStatusData)>) {
        self.car_status_handler = handler;
    }

    pub fn on_car_telemetry(&mut self, handler: Box<dyn Fn(&PacketCarTelemetryData)>) {
        self.car_telemetry_handler = handler;
    }

    pub fn on_event(&mut self, handler: Box<dyn Fn(&PacketEventData)>) {
        self.event_handler = handler;
    }

    pub fn on_final_classification(
        &mut self,
        handler: Box<dyn Fn(&PacketFinalClassificationData)>,
    ) {
        self.final_classification_handler = handler;
    }

    pub fn on_lap_data(&mut self, handler: Box<dyn Fn(&PacketLapData)>) {
        self.lap_data_handler = handler;
    }

    pub fn on_lobby_info(&mut self, handler: Box<dyn Fn(&PacketLobbyInfoData)>) {
        self.lobby_info_handler = handler;
    }

    pub fn on_motion_data(&mut self, handler: Box<dyn Fn(&PacketMotionData)>) {
        self.motion_data_handler = handler;
    }

    pub fn on_motion_ex_data(&mut self, handler: Box<dyn Fn(&PacketMotionExData)>) {
        self.motion_ex_data_handler = handler;
    }

    pub fn on_participants_data(&mut self, handler: Box<dyn Fn(&PacketParticipantsData)>) {
        self.participants_data_handler = handler;
    }

    pub fn on_session_data(&mut self, handler: Box<dyn Fn(&PacketSessionData)>) {
        self.session_data_handler = handler;
    }

    pub fn on_session_history_data(&mut self, handler: Box<dyn Fn(&PacketSessionHistoryData)>) {
        self.session_history_data_handler = handler;
    }

    pub fn on_tyre_sets_data(&mut self, handler: Box<dyn Fn(&PacketTyreSetsData)>) {
        self.tyre_sets_data_handler = handler;
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
                        0 => {
                            match PacketMotionData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.motion_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        1 => {
                            // match PacketSessionData::from_bytes(&self.buffer[..received]) {
                            //     Ok(packet) => (self.session_data_handler)(&packet),
                            //     Err(e) => eprintln!("{e:?}"),
                            // };
                        }
                        2 => {
                            match PacketLapData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.lap_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        3 => {
                            // match PacketEventData::from_bytes(&self.buffer[..received]) {
                            //     Ok(packet) => (self.event_handler)(&packet),
                            //     Err(e) => eprintln!("{e:?}"),
                            // };
                        }
                        4 => {
                            match PacketParticipantsData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.participants_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        5 => {
                            match PacketCarSetupData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.car_setup_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        6 => {
                            match PacketCarTelemetryData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.car_telemetry_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        7 => {
                            match PacketCarStatusData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.car_status_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        8 => {
                            match PacketFinalClassificationData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.final_classification_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        9 => {
                            match PacketLobbyInfoData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.lobby_info_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        10 => {
                            match PacketCarDamageData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.car_damage_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        11 => {
                            match PacketSessionHistoryData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.session_history_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        12 => {
                            match PacketTyreSetsData::from_bytes(&self.buffer[..received]) {
                                Ok(packet) => (self.tyre_sets_data_handler)(&packet),
                                Err(e) => eprintln!("{e:?}"),
                            };
                        }
                        13 => {
                            match PacketMotionExData::from_bytes(&self.buffer[..received]) {
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
