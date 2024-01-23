use super::packet_header::PacketHeader;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ParticipantData {
    pub ai_controlled: u8,     // 1 Byte
    pub driver_id: u8,         // 1 Byte
    pub network_id: u8,        // 1 Byte
    pub team_id: u8,           // 1 Byte
    pub my_team: u8,           // 1 Byte
    pub race_number: u8,       // 1 Byte
    pub nationality: u8,       // 1 Byte
    pub name: [u8; 48],        // 48 Bytes
    pub your_telemetry: u8,    // 1 Byte
    pub show_online_names: u8, // 1 Byte
    pub platform: u8,          // 1 Byte
} // 58 Bytes

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketParticipantsData {
    pub header: PacketHeader,                // 29 Bytes
    pub num_active_cars: u8,                 // 1 Byte
    pub participants: [ParticipantData; 22], // 1276 Bytes
} // 1306 Bytes
