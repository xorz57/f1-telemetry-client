use super::packet_header::PacketHeader;

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
