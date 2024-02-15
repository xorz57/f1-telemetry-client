use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CarStatusData {
    pub traction_control: u8,             // 1 Byte
    pub anti_lock_brakes: u8,             // 1 Byte
    pub fuel_mix: u8,                     // 1 Byte
    pub front_brake_bias: u8,             // 1 Byte
    pub pit_limiter_status: u8,           // 1 Byte
    pub fuel_in_tank: f32,                // 4 Bytes
    pub fuel_capacity: f32,               // 4 Bytes
    pub fuel_remaining_laps: f32,         // 4 Bytes
    pub max_rpm: u16,                     // 2 Bytes
    pub idle_rpm: u16,                    // 2 Bytes
    pub max_gears: u8,                    // 1 Byte
    pub drs_allowed: u8,                  // 1 Byte
    pub drs_activation_distance: u16,     // 2 Bytes
    pub actual_tyre_compound: u8,         // 1 Byte
    pub visual_tyre_compound: u8,         // 1 Byte
    pub tyres_age_laps: u8,               // 1 Byte
    pub vehicle_fia_flags: i8,            // 1 Byte
    pub engine_power_ice: f32,            // 4 Bytes
    pub engine_power_mguk: f32,           // 4 Bytes
    pub ers_store_energy: f32,            // 4 Bytes
    pub ers_deploy_mode: u8,              // 1 Byte
    pub ers_harvested_this_lap_mguk: f32, // 4 Bytes
    pub ers_harvested_this_lap_mguh: f32, // 4 Bytes
    pub ers_deployed_this_lap: f32,       // 4 Bytes
    pub network_paused: u8,               // 1 Byte
} // 55 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PacketCarStatusData {
    pub header: PacketHeader,                 // 29 Bytes
    pub car_status_data: [CarStatusData; 22], // 1210 Bytes
} // 1239 Bytes

impl CarStatusData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(CarStatusData {
            traction_control: cursor.read_u8()?,
            anti_lock_brakes: cursor.read_u8()?,
            fuel_mix: cursor.read_u8()?,
            front_brake_bias: cursor.read_u8()?,
            pit_limiter_status: cursor.read_u8()?,
            fuel_in_tank: cursor.read_f32::<LittleEndian>()?,
            fuel_capacity: cursor.read_f32::<LittleEndian>()?,
            fuel_remaining_laps: cursor.read_f32::<LittleEndian>()?,
            max_rpm: cursor.read_u16::<LittleEndian>()?,
            idle_rpm: cursor.read_u16::<LittleEndian>()?,
            max_gears: cursor.read_u8()?,
            drs_allowed: cursor.read_u8()?,
            drs_activation_distance: cursor.read_u16::<LittleEndian>()?,
            actual_tyre_compound: cursor.read_u8()?,
            visual_tyre_compound: cursor.read_u8()?,
            tyres_age_laps: cursor.read_u8()?,
            vehicle_fia_flags: cursor.read_i8()?,
            engine_power_ice: cursor.read_f32::<LittleEndian>()?,
            engine_power_mguk: cursor.read_f32::<LittleEndian>()?,
            ers_store_energy: cursor.read_f32::<LittleEndian>()?,
            ers_deploy_mode: cursor.read_u8()?,
            ers_harvested_this_lap_mguk: cursor.read_f32::<LittleEndian>()?,
            ers_harvested_this_lap_mguh: cursor.read_f32::<LittleEndian>()?,
            ers_deployed_this_lap: cursor.read_f32::<LittleEndian>()?,
            network_paused: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<CarStatusData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.traction_control)?;
        cursor.write_u8(self.anti_lock_brakes)?;
        cursor.write_u8(self.fuel_mix)?;
        cursor.write_u8(self.front_brake_bias)?;
        cursor.write_u8(self.pit_limiter_status)?;
        cursor.write_f32::<LittleEndian>(self.fuel_in_tank)?;
        cursor.write_f32::<LittleEndian>(self.fuel_capacity)?;
        cursor.write_f32::<LittleEndian>(self.fuel_remaining_laps)?;
        cursor.write_u16::<LittleEndian>(self.max_rpm)?;
        cursor.write_u16::<LittleEndian>(self.idle_rpm)?;
        cursor.write_u8(self.max_gears)?;
        cursor.write_u8(self.drs_allowed)?;
        cursor.write_u16::<LittleEndian>(self.drs_activation_distance)?;
        cursor.write_u8(self.actual_tyre_compound)?;
        cursor.write_u8(self.visual_tyre_compound)?;
        cursor.write_u8(self.tyres_age_laps)?;
        cursor.write_i8(self.vehicle_fia_flags)?;
        cursor.write_f32::<LittleEndian>(self.engine_power_ice)?;
        cursor.write_f32::<LittleEndian>(self.engine_power_mguk)?;
        cursor.write_f32::<LittleEndian>(self.ers_store_energy)?;
        cursor.write_u8(self.ers_deploy_mode)?;
        cursor.write_f32::<LittleEndian>(self.ers_harvested_this_lap_mguk)?;
        cursor.write_f32::<LittleEndian>(self.ers_harvested_this_lap_mguh)?;
        cursor.write_f32::<LittleEndian>(self.ers_deployed_this_lap)?;
        cursor.write_u8(self.network_paused)?;

        Ok(bytes)
    }
}

impl PacketCarStatusData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        Ok(PacketCarStatusData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            car_status_data: {
                let mut car_status_data: [CarStatusData; 22] = [CarStatusData::default(); 22];
                for i in 0..22 {
                    car_status_data[i] = CarStatusData::unserialize(
                        &bytes[size_of::<PacketHeader>() + i * size_of::<CarStatusData>()
                            ..size_of::<PacketHeader>() + (i + 1) * size_of::<CarStatusData>()],
                    )?;
                }
                car_status_data
            },
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketCarStatusData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        for element in self.car_status_data {
            cursor.write_all(&element.serialize()?)?;
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_status_data_serialization_deserialization() {
        // Create some sample car status data
        let original_car_status_data: CarStatusData = CarStatusData {
            traction_control: 1,
            anti_lock_brakes: 2,
            fuel_mix: 3,
            front_brake_bias: 4,
            pit_limiter_status: 5,
            fuel_in_tank: 0.1,
            fuel_capacity: 0.2,
            fuel_remaining_laps: 0.3,
            max_rpm: 1000,
            idle_rpm: 500,
            max_gears: 7,
            drs_allowed: 1,
            drs_activation_distance: 50,
            actual_tyre_compound: 1,
            visual_tyre_compound: 2,
            tyres_age_laps: 10,
            vehicle_fia_flags: -1,
            engine_power_ice: 300.0,
            engine_power_mguk: 200.0,
            ers_store_energy: 150.0,
            ers_deploy_mode: 1,
            ers_harvested_this_lap_mguk: 100.0,
            ers_harvested_this_lap_mguh: 50.0,
            ers_deployed_this_lap: 75.0,
            network_paused: 0,
        };

        // Serialize the data
        let serialized_data: Vec<u8> = original_car_status_data.serialize().unwrap();

        // Deserialize the serialized data
        let deserialized_car_status_data: CarStatusData =
            CarStatusData::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(original_car_status_data, deserialized_car_status_data);
    }

    #[test]
    fn test_packet_car_status_data_serialization_deserialization() {
        // Create some sample packet car status data
        let mut original_packet_car_status_data: PacketCarStatusData =
            PacketCarStatusData::default();
        original_packet_car_status_data.header.packet_format = 2021;
        original_packet_car_status_data.header.game_year = 21;
        original_packet_car_status_data.header.game_major_version = 1;
        original_packet_car_status_data.header.game_minor_version = 3;
        original_packet_car_status_data.header.packet_version = 1;
        original_packet_car_status_data.header.packet_id = 0;
        original_packet_car_status_data.header.session_uid = 123456789;
        original_packet_car_status_data.header.session_time = 123.456;
        original_packet_car_status_data.header.frame_identifier = 1000;
        original_packet_car_status_data
            .header
            .overall_frame_identifier = 5000;
        original_packet_car_status_data.header.player_car_index = 1;
        original_packet_car_status_data
            .header
            .secondary_player_car_index = 255;

        // Populate car status data array with some sample data
        for car_data in original_packet_car_status_data.car_status_data.iter_mut() {
            car_data.traction_control = 1;
            car_data.anti_lock_brakes = 2;
            car_data.fuel_mix = 3;
            car_data.front_brake_bias = 4;
            car_data.pit_limiter_status = 5;
            car_data.fuel_in_tank = 0.1;
            car_data.fuel_capacity = 0.2;
            car_data.fuel_remaining_laps = 0.3;
            car_data.max_rpm = 1000;
            car_data.idle_rpm = 500;
            car_data.max_gears = 7;
            car_data.drs_allowed = 1;
            car_data.drs_activation_distance = 50;
            car_data.actual_tyre_compound = 1;
            car_data.visual_tyre_compound = 2;
            car_data.tyres_age_laps = 10;
            car_data.vehicle_fia_flags = -1;
            car_data.engine_power_ice = 300.0;
            car_data.engine_power_mguk = 200.0;
            car_data.ers_store_energy = 150.0;
            car_data.ers_deploy_mode = 1;
            car_data.ers_harvested_this_lap_mguk = 100.0;
            car_data.ers_harvested_this_lap_mguh = 50.0;
            car_data.ers_deployed_this_lap = 75.0;
            car_data.network_paused = 0;
        }

        // Serialize the data
        let serialized_data: Vec<u8> = original_packet_car_status_data.serialize().unwrap();

        // Deserialize the serialized data
        let deserialized_packet_car_status_data: PacketCarStatusData =
            PacketCarStatusData::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(
            original_packet_car_status_data,
            deserialized_packet_car_status_data
        );
    }
}
