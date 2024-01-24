use f1_telemetry_client::packets::PacketCarTelemetryData;
use f1_telemetry_client::F1TelemetryClient;

fn main() {
    let mut client: F1TelemetryClient = F1TelemetryClient::new("0.0.0.0:20777");
    client.set_car_telemetry_handler(Box::new(|packet: &PacketCarTelemetryData| {
        println!("{packet:?}");
    }));
    client.run();
}
