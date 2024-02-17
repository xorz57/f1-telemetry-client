use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CarDamageData {
    pub tyres_wear: [f32; 4],        // 16 Bytes
    pub tyres_damage: [u8; 4],       // 4 Bytes
    pub brakes_damage: [u8; 4],      // 4 Bytes
    pub front_left_wing_damage: u8,  // 1 Byte
    pub front_right_wing_damage: u8, // 1 Byte
    pub rear_wing_damage: u8,        // 1 Byte
    pub floor_damage: u8,            // 1 Byte
    pub diffuser_damage: u8,         // 1 Byte
    pub sidepod_damage: u8,          // 1 Byte
    pub drs_fault: u8,               // 1 Byte
    pub ers_fault: u8,               // 1 Byte
    pub gearbox_damage: u8,          // 1 Byte
    pub engine_damage: u8,           // 1 Byte
    pub engine_mguh_wear: u8,        // 1 Byte
    pub engine_es_wear: u8,          // 1 Byte
    pub engine_ce_wear: u8,          // 1 Byte
    pub engine_ice_wear: u8,         // 1 Byte
    pub engine_mguk_wear: u8,        // 1 Byte
    pub engine_tc_wear: u8,          // 1 Byte
    pub engine_blown: u8,            // 1 Byte
    pub engine_seized: u8,           // 1 Byte
} // 42 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PacketCarDamageData {
    pub header: PacketHeader,                 // 29 Bytes
    pub car_damage_data: [CarDamageData; 22], // 924 Bytes
} // 953 Bytes

impl CarDamageData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(CarDamageData {
            tyres_wear: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
            tyres_damage: [
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
            ],
            brakes_damage: [
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
            ],
            front_left_wing_damage: cursor.read_u8()?,
            front_right_wing_damage: cursor.read_u8()?,
            rear_wing_damage: cursor.read_u8()?,
            floor_damage: cursor.read_u8()?,
            diffuser_damage: cursor.read_u8()?,
            sidepod_damage: cursor.read_u8()?,
            drs_fault: cursor.read_u8()?,
            ers_fault: cursor.read_u8()?,
            gearbox_damage: cursor.read_u8()?,
            engine_damage: cursor.read_u8()?,
            engine_mguh_wear: cursor.read_u8()?,
            engine_es_wear: cursor.read_u8()?,
            engine_ce_wear: cursor.read_u8()?,
            engine_ice_wear: cursor.read_u8()?,
            engine_mguk_wear: cursor.read_u8()?,
            engine_tc_wear: cursor.read_u8()?,
            engine_blown: cursor.read_u8()?,
            engine_seized: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<CarDamageData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        for element in self.tyres_wear {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        for element in self.tyres_damage {
            cursor.write_u8(element)?;
        }
        for element in self.brakes_damage {
            cursor.write_u8(element)?;
        }
        cursor.write_u8(self.front_left_wing_damage)?;
        cursor.write_u8(self.front_right_wing_damage)?;
        cursor.write_u8(self.rear_wing_damage)?;
        cursor.write_u8(self.floor_damage)?;
        cursor.write_u8(self.diffuser_damage)?;
        cursor.write_u8(self.sidepod_damage)?;
        cursor.write_u8(self.drs_fault)?;
        cursor.write_u8(self.ers_fault)?;
        cursor.write_u8(self.gearbox_damage)?;
        cursor.write_u8(self.engine_damage)?;
        cursor.write_u8(self.engine_mguh_wear)?;
        cursor.write_u8(self.engine_es_wear)?;
        cursor.write_u8(self.engine_ce_wear)?;
        cursor.write_u8(self.engine_ice_wear)?;
        cursor.write_u8(self.engine_mguk_wear)?;
        cursor.write_u8(self.engine_tc_wear)?;
        cursor.write_u8(self.engine_blown)?;
        cursor.write_u8(self.engine_seized)?;

        Ok(bytes)
    }
}

impl PacketCarDamageData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        Ok(PacketCarDamageData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            car_damage_data: {
                let mut car_damage_data: [CarDamageData; 22] = [CarDamageData::default(); 22];
                for i in 0..22 {
                    car_damage_data[i] = CarDamageData::unserialize(
                        &bytes[size_of::<PacketHeader>() + i * size_of::<CarDamageData>()
                            ..size_of::<PacketHeader>() + (i + 1) * size_of::<CarDamageData>()],
                    )?;
                }
                car_damage_data
            },
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketCarDamageData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        for element in self.car_damage_data {
            cursor.write_all(&element.serialize()?)?;
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_damage_data_serialization_deserialization() {
        let original_data: CarDamageData = CarDamageData {
            tyres_wear: [0.1f32, 0.2f32, 0.3f32, 0.4f32],
            tyres_damage: [10u8, 20u8, 30u8, 40u8],
            brakes_damage: [5u8, 10u8, 15u8, 20u8],
            front_left_wing_damage: 25u8,
            front_right_wing_damage: 30u8,
            rear_wing_damage: 35u8,
            floor_damage: 5u8,
            diffuser_damage: 8u8,
            sidepod_damage: 12u8,
            drs_fault: 0u8,
            ers_fault: 0u8,
            gearbox_damage: 15u8,
            engine_damage: 20u8,
            engine_mguh_wear: 25u8,
            engine_es_wear: 30u8,
            engine_ce_wear: 35u8,
            engine_ice_wear: 40u8,
            engine_mguk_wear: 45u8,
            engine_tc_wear: 50u8,
            engine_blown: 0u8,
            engine_seized: 0u8,
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: CarDamageData =
            CarDamageData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_car_damage_data_serialization_deserialization() {
        let original_packet: PacketCarDamageData = PacketCarDamageData {
            header: PacketHeader {
                packet_format: 2021u16,
                game_year: 21u8,
                game_major_version: 1u8,
                game_minor_version: 3u8,
                packet_version: 1u8,
                packet_id: 0u8,
                session_uid: 123456789u64,
                session_time: 123.456f32,
                frame_identifier: 1000u32,
                overall_frame_identifier: 5000u32,
                player_car_index: 1u8,
                secondary_player_car_index: 255u8,
            },
            car_damage_data: [CarDamageData {
                tyres_wear: [0.1f32, 0.2f32, 0.3f32, 0.4f32],
                tyres_damage: [10u8, 20u8, 30u8, 40u8],
                brakes_damage: [5u8, 10u8, 15u8, 20u8],
                front_left_wing_damage: 25u8,
                front_right_wing_damage: 30u8,
                rear_wing_damage: 35u8,
                floor_damage: 5u8,
                diffuser_damage: 8u8,
                sidepod_damage: 12u8,
                drs_fault: 0u8,
                ers_fault: 0u8,
                gearbox_damage: 15u8,
                engine_damage: 20u8,
                engine_mguh_wear: 25u8,
                engine_es_wear: 30u8,
                engine_ce_wear: 35u8,
                engine_ice_wear: 40u8,
                engine_mguk_wear: 45u8,
                engine_tc_wear: 50u8,
                engine_blown: 0u8,
                engine_seized: 0u8,
            }; 22],
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketCarDamageData =
            PacketCarDamageData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
