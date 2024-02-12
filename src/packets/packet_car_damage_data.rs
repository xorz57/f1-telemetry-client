use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
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

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketCarDamageData {
    pub header: PacketHeader,                 // 29 Bytes
    pub car_damage_data: [CarDamageData; 22], // 924 Bytes
} // 953 Bytes

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
        for car_damage_data in self.car_damage_data {
            cursor.write_all(&car_damage_data.serialize()?)?;
        }

        Ok(bytes)
    }
}
