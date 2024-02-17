use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CarSetupData {
    pub front_wing: u8,                 // 1 Byte
    pub rear_wing: u8,                  // 1 Byte
    pub on_throttle: u8,                // 1 Byte
    pub off_throttle: u8,               // 1 Byte
    pub front_camber: f32,              // 4 Bytes
    pub rear_camber: f32,               // 4 Bytes
    pub front_toe: f32,                 // 4 Bytes
    pub rear_toe: f32,                  // 4 Bytes
    pub front_suspension: u8,           // 1 Byte
    pub rear_suspension: u8,            // 1 Byte
    pub front_anti_roll_bar: u8,        // 1 Byte
    pub rear_anti_roll_bar: u8,         // 1 Byte
    pub front_suspension_height: u8,    // 1 Byte
    pub rear_suspension_height: u8,     // 1 Byte
    pub brake_pressure: u8,             // 1 Byte
    pub brake_bias: u8,                 // 1 Byte
    pub rear_left_tyre_pressure: f32,   // 4 Bytes
    pub rear_right_tyre_pressure: f32,  // 4 Bytes
    pub front_left_tyre_pressure: f32,  // 4 Bytes
    pub front_right_tyre_pressure: f32, // 4 Bytes
    pub ballast: u8,                    // 1 Byte
    pub fuel_load: f32,                 // 4 Bytes
} // 49 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PacketCarSetupData {
    pub header: PacketHeader,           // 29 Bytes
    pub car_setups: [CarSetupData; 22], // 1078 Bytes
} // 1107 Bytes

impl CarSetupData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(CarSetupData {
            front_wing: cursor.read_u8()?,
            rear_wing: cursor.read_u8()?,
            on_throttle: cursor.read_u8()?,
            off_throttle: cursor.read_u8()?,
            front_camber: cursor.read_f32::<LittleEndian>()?,
            rear_camber: cursor.read_f32::<LittleEndian>()?,
            front_toe: cursor.read_f32::<LittleEndian>()?,
            rear_toe: cursor.read_f32::<LittleEndian>()?,
            front_suspension: cursor.read_u8()?,
            rear_suspension: cursor.read_u8()?,
            front_anti_roll_bar: cursor.read_u8()?,
            rear_anti_roll_bar: cursor.read_u8()?,
            front_suspension_height: cursor.read_u8()?,
            rear_suspension_height: cursor.read_u8()?,
            brake_pressure: cursor.read_u8()?,
            brake_bias: cursor.read_u8()?,
            rear_left_tyre_pressure: cursor.read_f32::<LittleEndian>()?,
            rear_right_tyre_pressure: cursor.read_f32::<LittleEndian>()?,
            front_left_tyre_pressure: cursor.read_f32::<LittleEndian>()?,
            front_right_tyre_pressure: cursor.read_f32::<LittleEndian>()?,
            ballast: cursor.read_u8()?,
            fuel_load: cursor.read_f32::<LittleEndian>()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<CarSetupData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.front_wing)?;
        cursor.write_u8(self.rear_wing)?;
        cursor.write_u8(self.on_throttle)?;
        cursor.write_u8(self.off_throttle)?;
        cursor.write_f32::<LittleEndian>(self.front_camber)?;
        cursor.write_f32::<LittleEndian>(self.rear_camber)?;
        cursor.write_f32::<LittleEndian>(self.front_toe)?;
        cursor.write_f32::<LittleEndian>(self.rear_toe)?;
        cursor.write_u8(self.front_suspension)?;
        cursor.write_u8(self.rear_suspension)?;
        cursor.write_u8(self.front_anti_roll_bar)?;
        cursor.write_u8(self.rear_anti_roll_bar)?;
        cursor.write_u8(self.front_suspension_height)?;
        cursor.write_u8(self.rear_suspension_height)?;
        cursor.write_u8(self.brake_pressure)?;
        cursor.write_u8(self.brake_bias)?;
        cursor.write_f32::<LittleEndian>(self.rear_left_tyre_pressure)?;
        cursor.write_f32::<LittleEndian>(self.rear_right_tyre_pressure)?;
        cursor.write_f32::<LittleEndian>(self.front_left_tyre_pressure)?;
        cursor.write_f32::<LittleEndian>(self.front_right_tyre_pressure)?;
        cursor.write_u8(self.ballast)?;
        cursor.write_f32::<LittleEndian>(self.fuel_load)?;

        Ok(bytes)
    }
}

impl PacketCarSetupData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        Ok(PacketCarSetupData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            car_setups: {
                let mut car_setups: [CarSetupData; 22] = [CarSetupData::default(); 22];
                for i in 0..22 {
                    car_setups[i] = CarSetupData::unserialize(
                        &bytes[size_of::<PacketHeader>() + i * size_of::<CarSetupData>()
                            ..size_of::<PacketHeader>() + (i + 1) * size_of::<CarSetupData>()],
                    )?;
                }
                car_setups
            },
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<CarSetupData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        for element in self.car_setups {
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
    fn test_car_setup_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: CarSetupData = CarSetupData {
            front_wing: rng.gen(),
            rear_wing: rng.gen(),
            on_throttle: rng.gen(),
            off_throttle: rng.gen(),
            front_camber: rng.gen(),
            rear_camber: rng.gen(),
            front_toe: rng.gen(),
            rear_toe: rng.gen(),
            front_suspension: rng.gen(),
            rear_suspension: rng.gen(),
            front_anti_roll_bar: rng.gen(),
            rear_anti_roll_bar: rng.gen(),
            front_suspension_height: rng.gen(),
            rear_suspension_height: rng.gen(),
            brake_pressure: rng.gen(),
            brake_bias: rng.gen(),
            rear_left_tyre_pressure: rng.gen(),
            rear_right_tyre_pressure: rng.gen(),
            front_left_tyre_pressure: rng.gen(),
            front_right_tyre_pressure: rng.gen(),
            ballast: rng.gen(),
            fuel_load: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: CarSetupData = CarSetupData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_car_setup_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketCarSetupData = PacketCarSetupData {
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
            car_setups: [CarSetupData {
                front_wing: rng.gen(),
                rear_wing: rng.gen(),
                on_throttle: rng.gen(),
                off_throttle: rng.gen(),
                front_camber: rng.gen(),
                rear_camber: rng.gen(),
                front_toe: rng.gen(),
                rear_toe: rng.gen(),
                front_suspension: rng.gen(),
                rear_suspension: rng.gen(),
                front_anti_roll_bar: rng.gen(),
                rear_anti_roll_bar: rng.gen(),
                front_suspension_height: rng.gen(),
                rear_suspension_height: rng.gen(),
                brake_pressure: rng.gen(),
                brake_bias: rng.gen(),
                rear_left_tyre_pressure: rng.gen(),
                rear_right_tyre_pressure: rng.gen(),
                front_left_tyre_pressure: rng.gen(),
                front_right_tyre_pressure: rng.gen(),
                ballast: rng.gen(),
                fuel_load: rng.gen(),
            }; 22],
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketCarSetupData =
            PacketCarSetupData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
