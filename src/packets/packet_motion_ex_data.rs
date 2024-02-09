use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
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
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(PacketMotionExData {
            header: PacketHeader::from_bytes(&bytes[..size_of::<PacketHeader>()])?,
            suspension_position: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
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
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<PacketMotionExData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_all(&self.header.to_bytes()?)?;

        let suspension_position_array: [f32; 4] = self.suspension_position;
        for suspension_position in &suspension_position_array {
            cursor.write_f32::<LittleEndian>(*suspension_position)?;
        }

        let suspension_velocity_array: [f32; 4] = self.suspension_velocity;
        for suspension_velocity in &suspension_velocity_array {
            cursor.write_f32::<LittleEndian>(*suspension_velocity)?;
        }

        let suspension_acceleration_array: [f32; 4] = self.suspension_acceleration;
        for suspension_acceleration in &suspension_acceleration_array {
            cursor.write_f32::<LittleEndian>(*suspension_acceleration)?;
        }

        let wheel_speed_array: [f32; 4] = self.wheel_speed;
        for wheel_speed in &wheel_speed_array {
            cursor.write_f32::<LittleEndian>(*wheel_speed)?;
        }

        let wheel_slip_ratio_array: [f32; 4] = self.wheel_slip_ratio;
        for wheel_slip_ratio in &wheel_slip_ratio_array {
            cursor.write_f32::<LittleEndian>(*wheel_slip_ratio)?;
        }

        let wheel_slip_angle_array: [f32; 4] = self.wheel_slip_angle;
        for wheel_slip_angle in &wheel_slip_angle_array {
            cursor.write_f32::<LittleEndian>(*wheel_slip_angle)?;
        }

        let wheel_lat_force_array: [f32; 4] = self.wheel_lat_force;
        for wheel_lat_force in &wheel_lat_force_array {
            cursor.write_f32::<LittleEndian>(*wheel_lat_force)?;
        }

        let wheel_long_force_array: [f32; 4] = self.wheel_long_force;
        for wheel_long_force in &wheel_long_force_array {
            cursor.write_f32::<LittleEndian>(*wheel_long_force)?;
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

        let wheel_vert_force_array: [f32; 4] = self.wheel_vert_force;
        for wheel_vert_force in &wheel_vert_force_array {
            cursor.write_f32::<LittleEndian>(*wheel_vert_force)?;
        }

        Ok(buffer)
    }
}
