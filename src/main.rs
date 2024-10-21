use f1_telemetry_client::F1TelemetryClient;

fn main() {
    let mut client: F1TelemetryClient = F1TelemetryClient::new("0.0.0.0:20777");
    client.set_packet_car_damage_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_car_setup_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_car_status_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_car_telemetry_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_event_data_handler(Box::new(|packet| unsafe {
        let ed = packet.event_details;
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
    client.set_packet_final_classification_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_lap_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_lobby_info_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_motion_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_motion_ex_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_participants_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_session_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_session_history_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.set_packet_tyre_sets_data_handler(Box::new(|packet| {
        println!("{packet:?}");
    }));
    client.run();
}
