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
            header: {
                let pos: usize = size_of::<PacketHeader>();
                cursor.set_position(pos as u64);
                PacketHeader::unserialize(&bytes[..pos])?
            },
            weather: cursor.read_u8()?,
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
                let offset: usize = size_of::<PacketHeader>()
                    + 3 * size_of::<i8>()
                    + 10 * size_of::<u8>()
                    + 3 * size_of::<u16>();
                let pos: usize = offset + size_of::<[MarshalZone; 21]>();
                cursor.set_position(pos as u64);
                let mut marshal_zones: [MarshalZone; 21] = [MarshalZone::default(); 21];
                for i in 0..21 {
                    marshal_zones[i] = MarshalZone::unserialize(
                        &bytes[offset + i * size_of::<MarshalZone>()
                            ..offset + (i + 1) * size_of::<MarshalZone>()],
                    )?
                }

                marshal_zones
            },
            safety_car_status: cursor.read_u8()?,
            network_game: cursor.read_u8()?,
            num_weather_forecast_samples: cursor.read_u8()?,
            weather_forecast_samples: {
                let offset: usize = size_of::<PacketHeader>()
                    + 3 * size_of::<i8>()
                    + 13 * size_of::<u8>()
                    + 3 * size_of::<u16>()
                    + size_of::<[MarshalZone; 21]>();
                let pos: usize = offset + size_of::<[WeatherForecastSample; 56]>();
                cursor.set_position(pos as u64);

                let mut weather_forecast_samples: [WeatherForecastSample; 56] =
                    [WeatherForecastSample::default(); 56];
                for i in 0..56 {
                    weather_forecast_samples[i] = WeatherForecastSample::unserialize(
                        &bytes[offset + i * size_of::<WeatherForecastSample>()
                            ..offset + (i + 1) * size_of::<WeatherForecastSample>()],
                    )?;
                }
                weather_forecast_samples
            },
            forecast_accuracy: cursor.read_u8()?,
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

    #[test]
    fn test_packet_session_data_serialization_deserialization() {
        // Create some sample marshal zones
        let original_marshal_zones: [MarshalZone; 21] = [MarshalZone {
            zone_start: 10.0,
            zone_flag: 1,
        }; 21];

        // Create some sample weather forecast samples
        let original_weather_forecast_samples: [WeatherForecastSample; 56] =
            [WeatherForecastSample {
                session_type: 1,
                time_offset: 2,
                weather: 3,
                track_temperature: 20,
                track_temperature_change: 1,
                air_temperature: 25,
                air_temperature_change: -2,
                rain_percentage: 10,
            }; 56];

        // Create a sample packet session data
        let mut original_packet_session_data: PacketSessionData = PacketSessionData::default();
        original_packet_session_data.marshal_zones = original_marshal_zones;
        original_packet_session_data.weather_forecast_samples = original_weather_forecast_samples;

        // Serialize the data
        let serialized_data: Vec<u8> = original_packet_session_data.serialize().unwrap();

        // Deserialize the serialized data
        let deserialized_packet_session_data: PacketSessionData =
            PacketSessionData::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(
            original_packet_session_data,
            deserialized_packet_session_data
        );
    }
}
