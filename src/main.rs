use f1_telemetry_client::packets::EventDataDetails;
use f1_telemetry_client::packets::PacketCarDamageData;
use f1_telemetry_client::packets::PacketCarSetupData;
use f1_telemetry_client::packets::PacketCarStatusData;
use f1_telemetry_client::packets::PacketCarTelemetryData;
use f1_telemetry_client::packets::PacketEventData;
use f1_telemetry_client::packets::PacketFinalClassificationData;
use f1_telemetry_client::packets::PacketLapData;
use f1_telemetry_client::packets::PacketLobbyInfoData;
use f1_telemetry_client::packets::PacketMotionData;
use f1_telemetry_client::packets::PacketMotionExData;
use f1_telemetry_client::packets::PacketParticipantsData;
use f1_telemetry_client::packets::PacketSessionData;
use f1_telemetry_client::packets::PacketSessionHistoryData;
use f1_telemetry_client::packets::PacketTyreSetsData;
use f1_telemetry_client::F1TelemetryClient;

fn main() {
    let mut client: F1TelemetryClient = F1TelemetryClient::new("0.0.0.0:20777");
    client.on_car_damage(Box::new(|packet: &PacketCarDamageData| {
        println!("{packet:?}");
    }));
    client.on_car_setup(Box::new(|packet: &PacketCarSetupData| {
        println!("{packet:?}");
    }));
    client.on_car_status(Box::new(|packet: &PacketCarStatusData| {
        println!("{packet:?}");
    }));
    client.on_car_telemetry(Box::new(|packet: &PacketCarTelemetryData| {
        println!("{packet:?}");
    }));
    client.on_event(Box::new(|packet: &PacketEventData| unsafe {
        let ed: EventDataDetails = packet.event_details;
        match &packet.event_string_code {
            b"FTLP" => println!("{:?}", ed.fastest_lap),
            b"RTMT" => println!("{:?}", ed.retirement),
            b"TMPT" => println!("{:?}", ed.team_mate_in_pits),
            b"RCWN" => println!("{:?}", ed.race_winner),
            b"PENA" => println!("{:?}", ed.penalty),
            b"SPTP" => println!("{:?}", ed.speed_trap),
            b"STLG" => println!("{:?}", ed.start_lights),
            b"DTSV" => println!("{:?}", ed.drive_through_penalty_served),
            b"SGSV" => println!("{:?}", ed.stop_go_penalty_served),
            b"FLBK" => println!("{:?}", ed.flashback),
            b"BUTN" => println!("{:?}", ed.buttons),
            b"OVTK" => println!("{:?}", ed.overtake),
            b"SSTA" => println!("Session Started"),
            b"SEND" => println!("Session Ended"),
            b"DRSE" => println!("DRS Enabled"),
            b"DRSD" => println!("DRS Disabled"),
            b"CHQF" => println!("Chequered Flag"),
            b"LGOT" => println!("Lights Out"),
            b"RDFL" => println!("Red Flag"),
            _ => {}
        }
    }));
    client.on_final_classification(Box::new(|packet: &PacketFinalClassificationData| {
        println!("{packet:?}");
    }));
    client.on_lap_data(Box::new(|packet: &PacketLapData| {
        println!("{packet:?}");
    }));
    client.on_lobby_info(Box::new(|packet: &PacketLobbyInfoData| {
        println!("{packet:?}");
    }));
    client.on_motion_data(Box::new(|packet: &PacketMotionData| {
        println!("{packet:?}");
    }));
    client.on_motion_ex_data(Box::new(|packet: &PacketMotionExData| {
        println!("{packet:?}");
    }));
    client.on_participants_data(Box::new(|packet: &PacketParticipantsData| {
        println!("{packet:?}");
    }));
    client.on_session_data(Box::new(|packet: &PacketSessionData| {
        println!("{packet:?}");
    }));
    client.on_session_history_data(Box::new(|packet: &PacketSessionHistoryData| {
        println!("{packet:?}");
    }));
    client.on_tyre_sets_data(Box::new(|packet: &PacketTyreSetsData| {
        println!("{packet:?}");
    }));
    client.run();
}
