use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PacketMotionExData {
    pub header: PacketHeader,              // 29 Bytes
    pub suspension_position: [f32; 4],     // 16 Bytes
    pub suspension_velocity: [f32; 4],     // 16 Bytes
    pub suspension_acceleration: [f32; 4], // 16 Bytes
    pub wheel_speed: [f32; 4],             // 16 Bytes
    pub wheel_slip_ratio: [f32; 4],        // 16 Bytes
    pub wheel_slip_angle: [f32; 4],        // 16 Bytes
    pub wheel_lat_force: [f32; 4],         // 16 Bytes
    pub wheel_long_force: [f32; 4],        // 16 Bytes
    pub height_of_cog_above_ground: f32,   // 4 Bytes
    pub local_velocity_x: f32,             // 4 Bytes
    pub local_velocity_y: f32,             // 4 Bytes
    pub local_velocity_z: f32,             // 4 Bytes
    pub angular_velocity_x: f32,           // 4 Bytes
    pub angular_velocity_y: f32,           // 4 Bytes
    pub angular_velocity_z: f32,           // 4 Bytes
    pub angular_acceleration_x: f32,       // 4 Bytes
    pub angular_acceleration_y: f32,       // 4 Bytes
    pub angular_acceleration_z: f32,       // 4 Bytes
    pub front_wheels_angle: f32,           // 4 Bytes
    pub wheel_vert_force: [f32; 4],        // 16 Bytes
} // 217 Bytes

impl PacketMotionExData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(PacketMotionExData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            suspension_position: {
                let pos: usize = size_of::<PacketHeader>();
                cursor.set_position(pos as u64);
                [
                    cursor.read_f32::<LittleEndian>()?,
                    cursor.read_f32::<LittleEndian>()?,
                    cursor.read_f32::<LittleEndian>()?,
                    cursor.read_f32::<LittleEndian>()?,
                ]
            },
            suspension_velocity: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
            suspension_acceleration: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
            wheel_speed: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
            wheel_slip_ratio: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
            wheel_slip_angle: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
            wheel_lat_force: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
            wheel_long_force: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
            height_of_cog_above_ground: cursor.read_f32::<LittleEndian>()?,
            local_velocity_x: cursor.read_f32::<LittleEndian>()?,
            local_velocity_y: cursor.read_f32::<LittleEndian>()?,
            local_velocity_z: cursor.read_f32::<LittleEndian>()?,
            angular_velocity_x: cursor.read_f32::<LittleEndian>()?,
            angular_velocity_y: cursor.read_f32::<LittleEndian>()?,
            angular_velocity_z: cursor.read_f32::<LittleEndian>()?,
            angular_acceleration_x: cursor.read_f32::<LittleEndian>()?,
            angular_acceleration_y: cursor.read_f32::<LittleEndian>()?,
            angular_acceleration_z: cursor.read_f32::<LittleEndian>()?,
            front_wheels_angle: cursor.read_f32::<LittleEndian>()?,
            wheel_vert_force: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketMotionExData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        for element in self.suspension_position {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        for element in self.suspension_velocity {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        for element in self.suspension_acceleration {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        for element in self.wheel_speed {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        for element in self.wheel_slip_ratio {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        for element in self.wheel_slip_angle {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        for element in self.wheel_lat_force {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        for element in self.wheel_long_force {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        cursor.write_f32::<LittleEndian>(self.height_of_cog_above_ground)?;
        cursor.write_f32::<LittleEndian>(self.local_velocity_x)?;
        cursor.write_f32::<LittleEndian>(self.local_velocity_y)?;
        cursor.write_f32::<LittleEndian>(self.local_velocity_z)?;
        cursor.write_f32::<LittleEndian>(self.angular_velocity_x)?;
        cursor.write_f32::<LittleEndian>(self.angular_velocity_y)?;
        cursor.write_f32::<LittleEndian>(self.angular_velocity_z)?;
        cursor.write_f32::<LittleEndian>(self.angular_acceleration_x)?;
        cursor.write_f32::<LittleEndian>(self.angular_acceleration_y)?;
        cursor.write_f32::<LittleEndian>(self.angular_acceleration_z)?;
        cursor.write_f32::<LittleEndian>(self.front_wheels_angle)?;
        for element in self.wheel_vert_force {
            cursor.write_f32::<LittleEndian>(element)?;
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_motion_ex_data_serialization_deserialization() {
        // Create some sample packet motion ex data
        let mut original_packet_motion_ex_data: PacketMotionExData = PacketMotionExData::default();
        original_packet_motion_ex_data.header.packet_format = 2021u16;
        original_packet_motion_ex_data.header.game_year = 21u8;
        original_packet_motion_ex_data.header.game_major_version = 1u8;
        original_packet_motion_ex_data.header.game_minor_version = 3u8;
        original_packet_motion_ex_data.header.packet_version = 1u8;
        original_packet_motion_ex_data.header.packet_id = 0u8;
        original_packet_motion_ex_data.header.session_uid = 123456789u64;
        original_packet_motion_ex_data.header.session_time = 123.456f32;
        original_packet_motion_ex_data.header.frame_identifier = 1000u32;
        original_packet_motion_ex_data
            .header
            .overall_frame_identifier = 5000u32;
        original_packet_motion_ex_data.header.player_car_index = 1u8;
        original_packet_motion_ex_data
            .header
            .secondary_player_car_index = 255u8;

        // Populate arrays with some sample values
        for i in 0..4 {
            original_packet_motion_ex_data.suspension_position[i] = (i + 1) as f32 * 10.0;
            original_packet_motion_ex_data.suspension_velocity[i] = (i + 1) as f32 * 20.0;
            original_packet_motion_ex_data.suspension_acceleration[i] = (i + 1) as f32 * 30.0;
            original_packet_motion_ex_data.wheel_speed[i] = (i + 1) as f32 * 40.0;
            original_packet_motion_ex_data.wheel_slip_ratio[i] = (i + 1) as f32 * 50.0;
            original_packet_motion_ex_data.wheel_slip_angle[i] = (i + 1) as f32 * 60.0;
            original_packet_motion_ex_data.wheel_lat_force[i] = (i + 1) as f32 * 70.0;
            original_packet_motion_ex_data.wheel_long_force[i] = (i + 1) as f32 * 80.0;
            original_packet_motion_ex_data.wheel_vert_force[i] = (i + 1) as f32 * 90.0;
        }

        // Serialize the data
        let serialized_data: Vec<u8> = original_packet_motion_ex_data.serialize().unwrap();

        // Deserialize the serialized data
        let deserialized_packet_motion_ex_data: PacketMotionExData =
            PacketMotionExData::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(
            original_packet_motion_ex_data,
            deserialized_packet_motion_ex_data
        );
    }
}
