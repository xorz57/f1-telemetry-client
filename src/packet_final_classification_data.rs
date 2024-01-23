use super::packet_header::PacketHeader;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FinalClassificationData {
    pub position: u8,                  // 1 Byte
    pub num_laps: u8,                  // 1 Byte
    pub grid_position: u8,             // 1 Byte
    pub points: u8,                    // 1 Byte
    pub num_pit_stops: u8,             // 1 Byte
    pub result_status: u8,             // 1 Byte
    pub best_lap_time_in_ms: u32,      // 4 Bytes
    pub total_race_time: f64,          // 8 Bytes
    pub penalties_time: u8,            // 1 Byte
    pub num_penalties: u8,             // 1 Byte
    pub num_tyre_stints: u8,           // 1 Byte
    pub tyre_stints_actual: [u8; 8],   // 8 Bytes
    pub tyre_stints_visual: [u8; 8],   // 8 Bytes
    pub tyre_stints_end_laps: [u8; 8], // 8 Bytes
} // 45 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketFinalClassificationData {
    pub header: PacketHeader,                               // 29 Bytes
    pub num_cars: u8,                                       // 1 Byte
    pub classification_data: [FinalClassificationData; 22], // 990 Bytes
} // 1020 Bytes
