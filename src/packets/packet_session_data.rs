use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MarshalZone {
    pub zone_start: f32, // 4 Bytes
    pub zone_flag: i8,   // 1 Byte
} // 5 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub ai_difficulty: u8,                                     // 1 Byte
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

impl Default for PacketSessionData {
    fn default() -> Self {
        PacketSessionData {
            header: PacketHeader::default(),
            weather: 0u8,
            track_temperature: 0i8,
            air_temperature: 0i8,
            total_laps: 0u8,
            track_length: 0u16,
            session_type: 0u8,
            track_id: 0i8,
            formula: 0u8,
            session_time_left: 0u16,
            session_duration: 0u16,
            pit_speed_limit: 0u8,
            game_paused: 0u8,
            is_spectating: 0u8,
            spectator_car_index: 0u8,
            sli_pro_native_support: 0u8,
            num_marshal_zones: 0u8,
            marshal_zones: [MarshalZone::default(); 21],
            safety_car_status: 0u8,
            network_game: 0u8,
            num_weather_forecast_samples: 0u8,
            weather_forecast_samples: [WeatherForecastSample::default(); 56],
            forecast_accuracy: 0u8,
            ai_difficulty: 0u8,
            season_link_identifier: 0u32,
            weekend_link_identifier: 0u32,
            session_link_identifier: 0u32,
            pit_stop_window_ideal_lap: 0u8,
            pit_stop_window_latest_lap: 0u8,
            pit_stop_rejoin_position: 0u8,
            steering_assist: 0u8,
            braking_assist: 0u8,
            gearbox_assist: 0u8,
            pit_assist: 0u8,
            pit_release_assist: 0u8,
            ers_assist: 0u8,
            drs_assist: 0u8,
            dynamic_racing_line: 0u8,
            dynamic_racing_line_type: 0u8,
            game_mode: 0u8,
            rule_set: 0u8,
            time_of_day: 0u32,
            session_length: 0u8,
            speed_units_lead_player: 0u8,
            temperature_units_lead_player: 0u8,
            speed_units_secondary_player: 0u8,
            temperature_units_secondary_player: 0u8,
            num_safety_car_periods: 0u8,
            num_virtual_safety_car_periods: 0u8,
            num_red_flag_periods: 0u8,
        }
    }
}

impl MarshalZone {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(MarshalZone {
            zone_start: cursor.read_f32::<LittleEndian>()?,
            zone_flag: cursor.read_i8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<MarshalZone>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_f32::<LittleEndian>(self.zone_start)?;
        cursor.write_i8(self.zone_flag)?;

        Ok(bytes)
    }
}

impl WeatherForecastSample {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(WeatherForecastSample {
            session_type: cursor.read_u8()?,
            time_offset: cursor.read_u8()?,
            weather: cursor.read_u8()?,
            track_temperature: cursor.read_i8()?,
            track_temperature_change: cursor.read_i8()?,
            air_temperature: cursor.read_i8()?,
            air_temperature_change: cursor.read_i8()?,
            rain_percentage: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<WeatherForecastSample>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.session_type)?;
        cursor.write_u8(self.time_offset)?;
        cursor.write_u8(self.weather)?;
        cursor.write_i8(self.track_temperature)?;
        cursor.write_i8(self.track_temperature_change)?;
        cursor.write_i8(self.air_temperature)?;
        cursor.write_i8(self.air_temperature_change)?;
        cursor.write_u8(self.rain_percentage)?;

        Ok(bytes)
    }
}

impl PacketSessionData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(PacketSessionData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            weather: {
                let pos: usize = size_of::<PacketHeader>();
                cursor.set_position(pos as u64);
                cursor.read_u8()?
            },
            track_temperature: cursor.read_i8()?,
            air_temperature: cursor.read_i8()?,
            total_laps: cursor.read_u8()?,
            track_length: cursor.read_u16::<LittleEndian>()?,
            session_type: cursor.read_u8()?,
            track_id: cursor.read_i8()?,
            formula: cursor.read_u8()?,
            session_time_left: cursor.read_u16::<LittleEndian>()?,
            session_duration: cursor.read_u16::<LittleEndian>()?,
            pit_speed_limit: cursor.read_u8()?,
            game_paused: cursor.read_u8()?,
            is_spectating: cursor.read_u8()?,
            spectator_car_index: cursor.read_u8()?,
            sli_pro_native_support: cursor.read_u8()?,
            num_marshal_zones: cursor.read_u8()?,
            marshal_zones: {
                let mut marshal_zones: [MarshalZone; 21] = [MarshalZone::default(); 21];
                for i in 0..21 {
                    marshal_zones[i] = MarshalZone::unserialize(
                        &bytes[size_of::<PacketHeader>()
                            + 3 * size_of::<i8>()
                            + 10 * size_of::<u8>()
                            + 3 * size_of::<u16>()
                            + i * size_of::<MarshalZone>()
                            ..size_of::<PacketHeader>()
                                + 3 * size_of::<i8>()
                                + 10 * size_of::<u8>()
                                + 3 * size_of::<u16>()
                                + (i + 1) * size_of::<MarshalZone>()],
                    )?
                }

                marshal_zones
            },
            safety_car_status: {
                let pos: usize = size_of::<PacketHeader>()
                    + 3 * size_of::<i8>()
                    + 10 * size_of::<u8>()
                    + 3 * size_of::<u16>()
                    + size_of::<[MarshalZone; 21]>();
                cursor.set_position(pos as u64);
                cursor.read_u8()?
            },
            network_game: cursor.read_u8()?,
            num_weather_forecast_samples: cursor.read_u8()?,
            weather_forecast_samples: {
                let mut weather_forecast_samples: [WeatherForecastSample; 56] =
                    [WeatherForecastSample::default(); 56];
                for i in 0..56 {
                    weather_forecast_samples[i] = WeatherForecastSample::unserialize(
                        &bytes[size_of::<PacketHeader>()
                            + 3 * size_of::<i8>()
                            + 13 * size_of::<u8>()
                            + 3 * size_of::<u16>()
                            + size_of::<[MarshalZone; 21]>()
                            + i * size_of::<WeatherForecastSample>()
                            ..size_of::<PacketHeader>()
                                + 3 * size_of::<i8>()
                                + 13 * size_of::<u8>()
                                + 3 * size_of::<u16>()
                                + size_of::<[MarshalZone; 21]>()
                                + (i + 1) * size_of::<WeatherForecastSample>()],
                    )?;
                }
                weather_forecast_samples
            },
            forecast_accuracy: {
                let pos: usize = size_of::<PacketHeader>()
                    + 3 * size_of::<i8>()
                    + 13 * size_of::<u8>()
                    + 3 * size_of::<u16>()
                    + size_of::<[MarshalZone; 21]>()
                    + size_of::<[WeatherForecastSample; 56]>();
                cursor.set_position(pos as u64);
                cursor.read_u8()?
            },
            ai_difficulty: cursor.read_u8()?,
            season_link_identifier: cursor.read_u32::<LittleEndian>()?,
            weekend_link_identifier: cursor.read_u32::<LittleEndian>()?,
            session_link_identifier: cursor.read_u32::<LittleEndian>()?,
            pit_stop_window_ideal_lap: cursor.read_u8()?,
            pit_stop_window_latest_lap: cursor.read_u8()?,
            pit_stop_rejoin_position: cursor.read_u8()?,
            steering_assist: cursor.read_u8()?,
            braking_assist: cursor.read_u8()?,
            gearbox_assist: cursor.read_u8()?,
            pit_assist: cursor.read_u8()?,
            pit_release_assist: cursor.read_u8()?,
            ers_assist: cursor.read_u8()?,
            drs_assist: cursor.read_u8()?,
            dynamic_racing_line: cursor.read_u8()?,
            dynamic_racing_line_type: cursor.read_u8()?,
            game_mode: cursor.read_u8()?,
            rule_set: cursor.read_u8()?,
            time_of_day: cursor.read_u32::<LittleEndian>()?,
            session_length: cursor.read_u8()?,
            speed_units_lead_player: cursor.read_u8()?,
            temperature_units_lead_player: cursor.read_u8()?,
            speed_units_secondary_player: cursor.read_u8()?,
            temperature_units_secondary_player: cursor.read_u8()?,
            num_safety_car_periods: cursor.read_u8()?,
            num_virtual_safety_car_periods: cursor.read_u8()?,
            num_red_flag_periods: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<MarshalZone>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        cursor.write_u8(self.weather)?;
        cursor.write_i8(self.track_temperature)?;
        cursor.write_i8(self.air_temperature)?;
        cursor.write_u8(self.total_laps)?;
        cursor.write_u16::<LittleEndian>(self.track_length)?;
        cursor.write_u8(self.session_type)?;
        cursor.write_i8(self.track_id)?;
        cursor.write_u8(self.formula)?;
        cursor.write_u16::<LittleEndian>(self.session_time_left)?;
        cursor.write_u16::<LittleEndian>(self.session_duration)?;
        cursor.write_u8(self.pit_speed_limit)?;
        cursor.write_u8(self.game_paused)?;
        cursor.write_u8(self.is_spectating)?;
        cursor.write_u8(self.spectator_car_index)?;
        cursor.write_u8(self.sli_pro_native_support)?;
        cursor.write_u8(self.num_marshal_zones)?;
        for element in self.marshal_zones {
            cursor.write_all(&element.serialize()?)?;
        }
        cursor.write_u8(self.safety_car_status)?;
        cursor.write_u8(self.network_game)?;
        cursor.write_u8(self.num_weather_forecast_samples)?;
        for element in self.weather_forecast_samples {
            cursor.write_all(&element.serialize()?)?;
        }
        cursor.write_u8(self.forecast_accuracy)?;
        cursor.write_u8(self.ai_difficulty)?;
        cursor.write_u32::<LittleEndian>(self.season_link_identifier)?;
        cursor.write_u32::<LittleEndian>(self.weekend_link_identifier)?;
        cursor.write_u32::<LittleEndian>(self.session_link_identifier)?;
        cursor.write_u8(self.pit_stop_window_ideal_lap)?;
        cursor.write_u8(self.pit_stop_window_latest_lap)?;
        cursor.write_u8(self.pit_stop_rejoin_position)?;
        cursor.write_u8(self.steering_assist)?;
        cursor.write_u8(self.braking_assist)?;
        cursor.write_u8(self.gearbox_assist)?;
        cursor.write_u8(self.pit_assist)?;
        cursor.write_u8(self.pit_release_assist)?;
        cursor.write_u8(self.ers_assist)?;
        cursor.write_u8(self.drs_assist)?;
        cursor.write_u8(self.dynamic_racing_line)?;
        cursor.write_u8(self.dynamic_racing_line_type)?;
        cursor.write_u8(self.game_mode)?;
        cursor.write_u8(self.rule_set)?;
        cursor.write_u32::<LittleEndian>(self.time_of_day)?;
        cursor.write_u8(self.session_length)?;
        cursor.write_u8(self.speed_units_lead_player)?;
        cursor.write_u8(self.temperature_units_lead_player)?;
        cursor.write_u8(self.speed_units_secondary_player)?;
        cursor.write_u8(self.temperature_units_secondary_player)?;
        cursor.write_u8(self.num_safety_car_periods)?;
        cursor.write_u8(self.num_virtual_safety_car_periods)?;
        cursor.write_u8(self.num_red_flag_periods)?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_marshal_zone_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: MarshalZone = MarshalZone {
            zone_start: rng.gen(),
            zone_flag: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: MarshalZone = MarshalZone::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_weather_forecast_sample_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: WeatherForecastSample = WeatherForecastSample {
            session_type: rng.gen(),
            time_offset: rng.gen(),
            weather: rng.gen(),
            track_temperature: rng.gen(),
            track_temperature_change: rng.gen(),
            air_temperature: rng.gen(),
            air_temperature_change: rng.gen(),
            rain_percentage: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: WeatherForecastSample =
            WeatherForecastSample::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_session_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketSessionData = PacketSessionData {
            header: PacketHeader {
                packet_format: rng.gen(),
                game_year: rng.gen(),
                game_major_version: rng.gen(),
                game_minor_version: rng.gen(),
                packet_version: rng.gen(),
                packet_id: rng.gen(),
                session_uid: rng.gen(),
                session_time: rng.gen(),
                frame_identifier: rng.gen(),
                overall_frame_identifier: rng.gen(),
                player_car_index: rng.gen(),
                secondary_player_car_index: rng.gen(),
            },
            weather: rng.gen(),
            track_temperature: rng.gen(),
            air_temperature: rng.gen(),
            total_laps: rng.gen(),
            track_length: rng.gen(),
            session_type: rng.gen(),
            track_id: rng.gen(),
            formula: rng.gen(),
            session_time_left: rng.gen(),
            session_duration: rng.gen(),
            pit_speed_limit: rng.gen(),
            game_paused: rng.gen(),
            is_spectating: rng.gen(),
            spectator_car_index: rng.gen(),
            sli_pro_native_support: rng.gen(),
            num_marshal_zones: rng.gen(),
            marshal_zones: [MarshalZone {
                zone_start: rng.gen(),
                zone_flag: rng.gen(),
            }; 21],
            safety_car_status: rng.gen(),
            network_game: rng.gen(),
            num_weather_forecast_samples: rng.gen(),
            weather_forecast_samples: [WeatherForecastSample {
                session_type: rng.gen(),
                time_offset: rng.gen(),
                weather: rng.gen(),
                track_temperature: rng.gen(),
                track_temperature_change: rng.gen(),
                air_temperature: rng.gen(),
                air_temperature_change: rng.gen(),
                rain_percentage: rng.gen(),
            }; 56],
            forecast_accuracy: rng.gen(),
            ai_difficulty: rng.gen(),
            season_link_identifier: rng.gen(),
            weekend_link_identifier: rng.gen(),
            session_link_identifier: rng.gen(),
            pit_stop_window_ideal_lap: rng.gen(),
            pit_stop_window_latest_lap: rng.gen(),
            pit_stop_rejoin_position: rng.gen(),
            steering_assist: rng.gen(),
            braking_assist: rng.gen(),
            gearbox_assist: rng.gen(),
            pit_assist: rng.gen(),
            pit_release_assist: rng.gen(),
            ers_assist: rng.gen(),
            drs_assist: rng.gen(),
            dynamic_racing_line: rng.gen(),
            dynamic_racing_line_type: rng.gen(),
            game_mode: rng.gen(),
            rule_set: rng.gen(),
            time_of_day: rng.gen(),
            session_length: rng.gen(),
            speed_units_lead_player: rng.gen(),
            temperature_units_lead_player: rng.gen(),
            speed_units_secondary_player: rng.gen(),
            temperature_units_secondary_player: rng.gen(),
            num_safety_car_periods: rng.gen(),
            num_virtual_safety_car_periods: rng.gen(),
            num_red_flag_periods: rng.gen(),
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketSessionData =
            PacketSessionData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
