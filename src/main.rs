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
    client.set_car_damage_handler(Box::new(|packet: &PacketCarDamageData| {
        println!("{packet:?}");
    }));
    client.set_car_setup_handler(Box::new(|packet: &PacketCarSetupData| {
        println!("{packet:?}");
    }));
    client.set_car_status_handler(Box::new(|packet: &PacketCarStatusData| {
        println!("{packet:?}");
    }));
    client.set_car_telemetry_handler(Box::new(|packet: &PacketCarTelemetryData| {
        println!("{packet:?}");
    }));
    client.set_event_handler(Box::new(|_: &PacketEventData| {}));
    client.set_final_classification_handler(Box::new(|packet: &PacketFinalClassificationData| {
        println!("{packet:?}");
    }));
    client.set_lap_data_handler(Box::new(|packet: &PacketLapData| {
        println!("{packet:?}");
    }));
    client.set_lobby_info_handler(Box::new(|packet: &PacketLobbyInfoData| {
        println!("{packet:?}");
    }));
    client.set_motion_data_handler(Box::new(|packet: &PacketMotionData| {
        println!("{packet:?}");
    }));
    client.set_motion_ex_data_handler(Box::new(|packet: &PacketMotionExData| {
        println!("{packet:?}");
    }));
    client.set_participants_data_handler(Box::new(|packet: &PacketParticipantsData| {
        println!("{packet:?}");
    }));
    client.set_session_data_handler(Box::new(|packet: &PacketSessionData| {
        println!("{packet:?}");
    }));
    client.set_session_history_data_handler(Box::new(|packet: &PacketSessionHistoryData| {
        println!("{packet:?}");
    }));
    client.set_tyre_sets_data_handler(Box::new(|packet: &PacketTyreSetsData| {
        println!("{packet:?}");
    }));
    client.run();
}
