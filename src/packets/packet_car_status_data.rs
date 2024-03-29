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
    use rand::Rng;

    #[test]
    fn test_car_status_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: CarStatusData = CarStatusData {
            traction_control: rng.gen(),
            anti_lock_brakes: rng.gen(),
            fuel_mix: rng.gen(),
            front_brake_bias: rng.gen(),
            pit_limiter_status: rng.gen(),
            fuel_in_tank: rng.gen(),
            fuel_capacity: rng.gen(),
            fuel_remaining_laps: rng.gen(),
            max_rpm: rng.gen(),
            idle_rpm: rng.gen(),
            max_gears: rng.gen(),
            drs_allowed: rng.gen(),
            drs_activation_distance: rng.gen(),
            actual_tyre_compound: rng.gen(),
            visual_tyre_compound: rng.gen(),
            tyres_age_laps: rng.gen(),
            vehicle_fia_flags: rng.gen(),
            engine_power_ice: rng.gen(),
            engine_power_mguk: rng.gen(),
            ers_store_energy: rng.gen(),
            ers_deploy_mode: rng.gen(),
            ers_harvested_this_lap_mguk: rng.gen(),
            ers_harvested_this_lap_mguh: rng.gen(),
            ers_deployed_this_lap: rng.gen(),
            network_paused: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: CarStatusData =
            CarStatusData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_car_status_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketCarStatusData = PacketCarStatusData {
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
            car_status_data: [CarStatusData {
                traction_control: rng.gen(),
                anti_lock_brakes: rng.gen(),
                fuel_mix: rng.gen(),
                front_brake_bias: rng.gen(),
                pit_limiter_status: rng.gen(),
                fuel_in_tank: rng.gen(),
                fuel_capacity: rng.gen(),
                fuel_remaining_laps: rng.gen(),
                max_rpm: rng.gen(),
                idle_rpm: rng.gen(),
                max_gears: rng.gen(),
                drs_allowed: rng.gen(),
                drs_activation_distance: rng.gen(),
                actual_tyre_compound: rng.gen(),
                visual_tyre_compound: rng.gen(),
                tyres_age_laps: rng.gen(),
                vehicle_fia_flags: rng.gen(),
                engine_power_ice: rng.gen(),
                engine_power_mguk: rng.gen(),
                ers_store_energy: rng.gen(),
                ers_deploy_mode: rng.gen(),
                ers_harvested_this_lap_mguk: rng.gen(),
                ers_harvested_this_lap_mguh: rng.gen(),
                ers_deployed_this_lap: rng.gen(),
                network_paused: rng.gen(),
            }; 22],
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketCarStatusData =
            PacketCarStatusData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
