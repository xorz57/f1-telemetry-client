use super::packet_header::PacketHeader;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LapData {
    pub last_lap_time_in_ms: u32,            // 4 Bytes
    pub current_lap_time_in_ms: u32,         // 4 Bytes
    pub sector1_time_in_ms: u16,             // 2 Bytes
    pub sector1_time_minutes: u8,            // 1 Byte
    pub sector2_time_in_ms: u16,             // 2 Bytes
    pub sector2_time_minutes: u8,            // 1 Byte
    pub delta_to_car_in_front_in_ms: u16,    // 2 Bytes
    pub delta_to_race_leader_in_ms: u16,     // 2 Bytes
    pub lap_distance: f32,                   // 4 Bytes
    pub total_distance: f32,                 // 4 Bytes
    pub safety_car_delta: f32,               // 4 Bytes
    pub car_position: u8,                    // 1 Byte
    pub current_lap_num: u8,                 // 1 Byte
    pub pit_status: u8,                      // 1 Byte
    pub num_pit_stops: u8,                   // 1 Byte
    pub sector: u8,                          // 1 Byte
    pub current_lap_invalid: u8,             // 1 Byte
    pub penalties: u8,                       // 1 Byte
    pub total_warnings: u8,                  // 1 Byte
    pub corner_cutting_warnings: u8,         // 1 Byte
    pub num_unserved_drive_through_pens: u8, // 1 Byte
    pub num_unserved_stop_go_pens: u8,       // 1 Byte
    pub grid_position: u8,                   // 1 Byte
    pub driver_status: u8,                   // 1 Byte
    pub result_status: u8,                   // 1 Byte
    pub pit_lane_timer_active: u8,           // 1 Byte
    pub pit_lane_time_in_lane_in_ms: u16,    // 2 Bytes
    pub pit_stop_timer_in_ms: u16,           // 2 Bytes
    pub pit_stop_should_serve_pen: u8,       // 1 Byte
} // 50 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketLapData {
    pub header: PacketHeader,         // 29 Bytes
    pub lap_data: [LapData; 22],      // 1100 Bytes
    pub time_trial_pb_car_idx: u8,    // 1 Byte
    pub time_trial_rival_car_idx: u8, // 1 Byte
} // 1131 Bytes
