use super::packet_header::PacketHeader;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MarshalZone {
    pub zone_start: f32, // 4 Bytes
    pub zone_flag: i8,   // 1 Byte
} // 5 Bytes

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct WeatherForecastSample {
    pub session_type: u8,             // 1 Byte
    pub time_offset: u8,              // 1 Byte
    pub weather: u8,                  // 1 Byte
    pub track_temperature: i8,        // 1 Byte
    pub track_temperature_change: i8, // 1 Byte
    pub air_temperature: i8,          // 1 Byte
    pub air_temperature_change: i8,   // 1 Byte
    pub rain_percentage: u8,          // 1 Byte
} // 8 Bytes

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketSessionData {
    pub header: PacketHeader,                                  // 29 Bytes
    pub weather: u8,                                           // 1 Byte
    pub track_temperature: i8,                                 // 1 Byte
    pub air_temperature: i8,                                   // 1 Byte
    pub total_laps: u8,                                        // 1 Byte
    pub track_length: u16,                                     // 2 Bytes
    pub session_type: u8,                                      // 1 Byte
    pub track_id: i8,                                          // 1 Byte
    pub formula: u8,                                           // 1 Byte
    pub session_time_left: u16,                                // 2 Bytes
    pub session_duration: u16,                                 // 2 Bytes
    pub pit_speed_limit: u8,                                   // 1 Byte
    pub game_paused: u8,                                       // 1 Byte
    pub is_spectating: u8,                                     // 1 Byte
    pub spectator_car_index: u8,                               // 1 Byte
    pub sli_pro_native_support: u8,                            // 1 Byte
    pub num_marshal_zones: u8,                                 // 1 Byte
    pub marshal_zones: [MarshalZone; 21],                      // 105 Bytes
    pub safety_car_status: u8,                                 // 1 Byte
    pub network_game: u8,                                      // 1 Byte
    pub num_weather_forecast_samples: u8,                      // 1 Byte
    pub weather_forecast_samples: [WeatherForecastSample; 56], // 448 Bytes
    pub forecast_accuracy: u8,                                 // 1 Byte
    pub ai_fifficulty: u8,                                     // 1 Byte
    pub season_link_identifier: u32,                           // 4 Bytes
    pub weekend_link_identifier: u32,                          // 4 Bytes
    pub session_link_identifier: u32,                          // 4 Bytes
    pub pit_stop_window_ideal_lap: u8,                         // 1 Byte
    pub pit_stop_window_latest_lap: u8,                        // 1 Byte
    pub pit_stop_rejoin_position: u8,                          // 1 Byte
    pub steering_assist: u8,                                   // 1 Byte
    pub braking_assist: u8,                                    // 1 Byte
    pub gearbox_assist: u8,                                    // 1 Byte
    pub pit_assist: u8,                                        // 1 Byte
    pub pit_release_assist: u8,                                // 1 Byte
    pub ers_assist: u8,                                        // 1 Byte
    pub drs_assist: u8,                                        // 1 Byte
    pub dynamic_racing_line: u8,                               // 1 Byte
    pub dynamic_racing_line_type: u8,                          // 1 Byte
    pub game_mode: u8,                                         // 1 Byte
    pub rule_set: u8,                                          // 1 Byte
    pub time_of_day: u32,                                      // 4 Bytes
    pub session_length: u8,                                    // 1 Byte
    pub speed_units_lead_player: u8,                           // 1 Byte
    pub temperature_units_lead_player: u8,                     // 1 Byte
    pub speed_units_secondary_player: u8,                      // 1 Byte
    pub temperature_units_secondary_player: u8,                // 1 Byte
    pub num_safety_car_periods: u8,                            // 1 Byte
    pub num_virtual_safety_car_periods: u8,                    // 1 Byte
    pub num_red_flag_periods: u8,                              // 1 Byte
} // 644 Bytes
