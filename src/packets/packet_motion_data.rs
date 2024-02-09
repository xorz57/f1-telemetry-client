use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
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

impl CarMotionData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
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
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<CarMotionData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

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

        Ok(buffer)
    }
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketMotionData {
    pub header: PacketHeader,
    pub car_motion_data: [CarMotionData; 22],
} // 1349 Bytes

impl PacketMotionData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        Ok(PacketMotionData {
            header: PacketHeader::from_bytes(&bytes[..size_of::<PacketHeader>()])?,
            car_motion_data: {
                let mut car_motion_data: [CarMotionData; 22] = [CarMotionData::default(); 22];
                for i in 0..22 {
                    car_motion_data[i] = CarMotionData::from_bytes(
                        &bytes[size_of::<PacketHeader>() + i * size_of::<CarMotionData>()
                            ..size_of::<PacketHeader>() + (i + 1) * size_of::<CarMotionData>()],
                    )?;
                }
                car_motion_data
            },
        })
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<PacketMotionData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_all(&self.header.to_bytes()?)?;

        for car_motion_data in &self.car_motion_data {
            cursor.write_all(&car_motion_data.to_bytes()?)?;
        }

        Ok(buffer)
    }
}
