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
    use rand::Rng;

    #[test]
    fn test_packet_motion_ex_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketMotionExData = PacketMotionExData {
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
            suspension_position: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            suspension_velocity: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            suspension_acceleration: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            wheel_speed: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            wheel_slip_ratio: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            wheel_slip_angle: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            wheel_lat_force: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            wheel_long_force: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            height_of_cog_above_ground: rng.gen(),
            local_velocity_x: rng.gen(),
            local_velocity_y: rng.gen(),
            local_velocity_z: rng.gen(),
            angular_velocity_x: rng.gen(),
            angular_velocity_y: rng.gen(),
            angular_velocity_z: rng.gen(),
            angular_acceleration_x: rng.gen(),
            angular_acceleration_y: rng.gen(),
            angular_acceleration_z: rng.gen(),
            front_wheels_angle: rng.gen(),
            wheel_vert_force: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketMotionExData =
            PacketMotionExData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
