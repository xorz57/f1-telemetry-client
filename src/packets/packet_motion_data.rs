use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CarMotionData {
    pub world_position_x: f32,     // 4 Bytes
    pub world_position_y: f32,     // 4 Bytes
    pub world_position_z: f32,     // 4 Bytes
    pub world_velocity_x: f32,     // 4 Bytes
    pub world_velocity_y: f32,     // 4 Bytes
    pub world_velocity_z: f32,     // 4 Bytes
    pub world_forward_dir_x: i16,  // 2 Bytes
    pub world_forward_dir_y: i16,  // 2 Bytes
    pub world_forward_dir_z: i16,  // 2 Bytes
    pub world_right_dir_x: i16,    // 2 Bytes
    pub world_right_dir_y: i16,    // 2 Bytes
    pub world_right_dir_z: i16,    // 2 Bytes
    pub g_force_lateral: f32,      // 4 Bytes
    pub g_force_longitudinal: f32, // 4 Bytes
    pub g_force_vertical: f32,     // 4 Bytes
    pub yaw: f32,                  // 4 Bytes
    pub pitch: f32,                // 4 Bytes
    pub roll: f32,                 // 4 Bytes
} // 60 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PacketMotionData {
    pub header: PacketHeader,
    pub car_motion_data: [CarMotionData; 22],
} // 1349 Bytes

impl CarMotionData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(CarMotionData {
            world_position_x: cursor.read_f32::<LittleEndian>()?,
            world_position_y: cursor.read_f32::<LittleEndian>()?,
            world_position_z: cursor.read_f32::<LittleEndian>()?,
            world_velocity_x: cursor.read_f32::<LittleEndian>()?,
            world_velocity_y: cursor.read_f32::<LittleEndian>()?,
            world_velocity_z: cursor.read_f32::<LittleEndian>()?,
            world_forward_dir_x: cursor.read_i16::<LittleEndian>()?,
            world_forward_dir_y: cursor.read_i16::<LittleEndian>()?,
            world_forward_dir_z: cursor.read_i16::<LittleEndian>()?,
            world_right_dir_x: cursor.read_i16::<LittleEndian>()?,
            world_right_dir_y: cursor.read_i16::<LittleEndian>()?,
            world_right_dir_z: cursor.read_i16::<LittleEndian>()?,
            g_force_lateral: cursor.read_f32::<LittleEndian>()?,
            g_force_longitudinal: cursor.read_f32::<LittleEndian>()?,
            g_force_vertical: cursor.read_f32::<LittleEndian>()?,
            yaw: cursor.read_f32::<LittleEndian>()?,
            pitch: cursor.read_f32::<LittleEndian>()?,
            roll: cursor.read_f32::<LittleEndian>()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<CarMotionData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_f32::<LittleEndian>(self.world_position_x)?;
        cursor.write_f32::<LittleEndian>(self.world_position_y)?;
        cursor.write_f32::<LittleEndian>(self.world_position_z)?;
        cursor.write_f32::<LittleEndian>(self.world_velocity_x)?;
        cursor.write_f32::<LittleEndian>(self.world_velocity_y)?;
        cursor.write_f32::<LittleEndian>(self.world_velocity_z)?;
        cursor.write_i16::<LittleEndian>(self.world_forward_dir_x)?;
        cursor.write_i16::<LittleEndian>(self.world_forward_dir_y)?;
        cursor.write_i16::<LittleEndian>(self.world_forward_dir_z)?;
        cursor.write_i16::<LittleEndian>(self.world_right_dir_x)?;
        cursor.write_i16::<LittleEndian>(self.world_right_dir_y)?;
        cursor.write_i16::<LittleEndian>(self.world_right_dir_z)?;
        cursor.write_f32::<LittleEndian>(self.g_force_lateral)?;
        cursor.write_f32::<LittleEndian>(self.g_force_longitudinal)?;
        cursor.write_f32::<LittleEndian>(self.g_force_vertical)?;
        cursor.write_f32::<LittleEndian>(self.yaw)?;
        cursor.write_f32::<LittleEndian>(self.pitch)?;
        cursor.write_f32::<LittleEndian>(self.roll)?;

        Ok(bytes)
    }
}

impl PacketMotionData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        Ok(PacketMotionData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            car_motion_data: {
                let mut car_motion_data: [CarMotionData; 22] = [CarMotionData::default(); 22];
                for i in 0..22 {
                    car_motion_data[i] = CarMotionData::unserialize(
                        &bytes[size_of::<PacketHeader>() + i * size_of::<CarMotionData>()
                            ..size_of::<PacketHeader>() + (i + 1) * size_of::<CarMotionData>()],
                    )?;
                }
                car_motion_data
            },
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketMotionData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        for element in self.car_motion_data {
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
    fn test_car_motion_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: CarMotionData = CarMotionData {
            world_position_x: rng.gen(),
            world_position_y: rng.gen(),
            world_position_z: rng.gen(),
            world_velocity_x: rng.gen(),
            world_velocity_y: rng.gen(),
            world_velocity_z: rng.gen(),
            world_forward_dir_x: rng.gen(),
            world_forward_dir_y: rng.gen(),
            world_forward_dir_z: rng.gen(),
            world_right_dir_x: rng.gen(),
            world_right_dir_y: rng.gen(),
            world_right_dir_z: rng.gen(),
            g_force_lateral: rng.gen(),
            g_force_longitudinal: rng.gen(),
            g_force_vertical: rng.gen(),
            yaw: rng.gen(),
            pitch: rng.gen(),
            roll: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: CarMotionData =
            CarMotionData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_motion_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketMotionData = PacketMotionData {
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
            car_motion_data: [CarMotionData {
                world_position_x: rng.gen(),
                world_position_y: rng.gen(),
                world_position_z: rng.gen(),
                world_velocity_x: rng.gen(),
                world_velocity_y: rng.gen(),
                world_velocity_z: rng.gen(),
                world_forward_dir_x: rng.gen(),
                world_forward_dir_y: rng.gen(),
                world_forward_dir_z: rng.gen(),
                world_right_dir_x: rng.gen(),
                world_right_dir_y: rng.gen(),
                world_right_dir_z: rng.gen(),
                g_force_lateral: rng.gen(),
                g_force_longitudinal: rng.gen(),
                g_force_vertical: rng.gen(),
                yaw: rng.gen(),
                pitch: rng.gen(),
                roll: rng.gen(),
            }; 22],
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketMotionData =
            PacketMotionData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
