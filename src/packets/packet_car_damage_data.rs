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
    use rand::Rng;

    #[test]
    fn test_car_damage_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: CarDamageData = CarDamageData {
            tyres_wear: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            tyres_damage: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            brakes_damage: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            front_left_wing_damage: rng.gen(),
            front_right_wing_damage: rng.gen(),
            rear_wing_damage: rng.gen(),
            floor_damage: rng.gen(),
            diffuser_damage: rng.gen(),
            sidepod_damage: rng.gen(),
            drs_fault: rng.gen(),
            ers_fault: rng.gen(),
            gearbox_damage: rng.gen(),
            engine_damage: rng.gen(),
            engine_mguh_wear: rng.gen(),
            engine_es_wear: rng.gen(),
            engine_ce_wear: rng.gen(),
            engine_ice_wear: rng.gen(),
            engine_mguk_wear: rng.gen(),
            engine_tc_wear: rng.gen(),
            engine_blown: rng.gen(),
            engine_seized: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: CarDamageData =
            CarDamageData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_car_damage_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketCarDamageData = PacketCarDamageData {
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
            car_damage_data: [CarDamageData {
                tyres_wear: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
                tyres_damage: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
                brakes_damage: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
                front_left_wing_damage: rng.gen(),
                front_right_wing_damage: rng.gen(),
                rear_wing_damage: rng.gen(),
                floor_damage: rng.gen(),
                diffuser_damage: rng.gen(),
                sidepod_damage: rng.gen(),
                drs_fault: rng.gen(),
                ers_fault: rng.gen(),
                gearbox_damage: rng.gen(),
                engine_damage: rng.gen(),
                engine_mguh_wear: rng.gen(),
                engine_es_wear: rng.gen(),
                engine_ce_wear: rng.gen(),
                engine_ice_wear: rng.gen(),
                engine_mguk_wear: rng.gen(),
                engine_tc_wear: rng.gen(),
                engine_blown: rng.gen(),
                engine_seized: rng.gen(),
            }; 22],
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketCarDamageData =
            PacketCarDamageData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
