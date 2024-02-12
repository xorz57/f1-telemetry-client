use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LapData {
    pub last_lap_time_in_ms: u32,            // 4 Bytes
    pub current_lap_time_in_ms: u32,         // 4 Bytes
    pub sector1_time_in_ms: u16,             // 2 Bytes
    pub sector1_time_minutes: u8,            // 1 Byte
    pub sector2_time_in_ms: u16,             // 2 Bytes
    pub sector2_time_minutes: u8,            // 1 Byte
    pub delta_to_car_in_front_in_ms: u16,    // 2 Bytes
    pub delta_to_race_leader_in_ms: u16,     // 2 Bytes
    pub lap_distance: f32,                   // 4 Bytes
    pub total_distance: f32,                 // 4 Bytes
    pub safety_car_delta: f32,               // 4 Bytes
    pub car_position: u8,                    // 1 Byte
    pub current_lap_num: u8,                 // 1 Byte
    pub pit_status: u8,                      // 1 Byte
    pub num_pit_stops: u8,                   // 1 Byte
    pub sector: u8,                          // 1 Byte
    pub current_lap_invalid: u8,             // 1 Byte
    pub penalties: u8,                       // 1 Byte
    pub total_warnings: u8,                  // 1 Byte
    pub corner_cutting_warnings: u8,         // 1 Byte
    pub num_unserved_drive_through_pens: u8, // 1 Byte
    pub num_unserved_stop_go_pens: u8,       // 1 Byte
    pub grid_position: u8,                   // 1 Byte
    pub driver_status: u8,                   // 1 Byte
    pub result_status: u8,                   // 1 Byte
    pub pit_lane_timer_active: u8,           // 1 Byte
    pub pit_lane_time_in_lane_in_ms: u16,    // 2 Bytes
    pub pit_stop_timer_in_ms: u16,           // 2 Bytes
    pub pit_stop_should_serve_pen: u8,       // 1 Byte
} // 50 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketLapData {
    pub header: PacketHeader,         // 29 Bytes
    pub lap_data: [LapData; 22],      // 1100 Bytes
    pub time_trial_pb_car_idx: u8,    // 1 Byte
    pub time_trial_rival_car_idx: u8, // 1 Byte
} // 1131 Bytes

impl LapData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(LapData {
            last_lap_time_in_ms: cursor.read_u32::<LittleEndian>()?,
            current_lap_time_in_ms: cursor.read_u32::<LittleEndian>()?,
            sector1_time_in_ms: cursor.read_u16::<LittleEndian>()?,
            sector1_time_minutes: cursor.read_u8()?,
            sector2_time_in_ms: cursor.read_u16::<LittleEndian>()?,
            sector2_time_minutes: cursor.read_u8()?,
            delta_to_car_in_front_in_ms: cursor.read_u16::<LittleEndian>()?,
            delta_to_race_leader_in_ms: cursor.read_u16::<LittleEndian>()?,
            lap_distance: cursor.read_f32::<LittleEndian>()?,
            total_distance: cursor.read_f32::<LittleEndian>()?,
            safety_car_delta: cursor.read_f32::<LittleEndian>()?,
            car_position: cursor.read_u8()?,
            current_lap_num: cursor.read_u8()?,
            pit_status: cursor.read_u8()?,
            num_pit_stops: cursor.read_u8()?,
            sector: cursor.read_u8()?,
            current_lap_invalid: cursor.read_u8()?,
            penalties: cursor.read_u8()?,
            total_warnings: cursor.read_u8()?,
            corner_cutting_warnings: cursor.read_u8()?,
            num_unserved_drive_through_pens: cursor.read_u8()?,
            num_unserved_stop_go_pens: cursor.read_u8()?,
            grid_position: cursor.read_u8()?,
            driver_status: cursor.read_u8()?,
            result_status: cursor.read_u8()?,
            pit_lane_timer_active: cursor.read_u8()?,
            pit_lane_time_in_lane_in_ms: cursor.read_u16::<LittleEndian>()?,
            pit_stop_timer_in_ms: cursor.read_u16::<LittleEndian>()?,
            pit_stop_should_serve_pen: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<LapData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_u32::<LittleEndian>(self.last_lap_time_in_ms)?;
        cursor.write_u32::<LittleEndian>(self.current_lap_time_in_ms)?;
        cursor.write_u16::<LittleEndian>(self.sector1_time_in_ms)?;
        cursor.write_u8(self.sector1_time_minutes)?;
        cursor.write_u16::<LittleEndian>(self.sector2_time_in_ms)?;
        cursor.write_u8(self.sector2_time_minutes)?;
        cursor.write_u16::<LittleEndian>(self.delta_to_car_in_front_in_ms)?;
        cursor.write_u16::<LittleEndian>(self.delta_to_race_leader_in_ms)?;
        cursor.write_f32::<LittleEndian>(self.lap_distance)?;
        cursor.write_f32::<LittleEndian>(self.total_distance)?;
        cursor.write_f32::<LittleEndian>(self.safety_car_delta)?;
        cursor.write_u8(self.car_position)?;
        cursor.write_u8(self.current_lap_num)?;
        cursor.write_u8(self.pit_status)?;
        cursor.write_u8(self.num_pit_stops)?;
        cursor.write_u8(self.sector)?;
        cursor.write_u8(self.current_lap_invalid)?;
        cursor.write_u8(self.penalties)?;
        cursor.write_u8(self.total_warnings)?;
        cursor.write_u8(self.corner_cutting_warnings)?;
        cursor.write_u8(self.num_unserved_drive_through_pens)?;
        cursor.write_u8(self.num_unserved_stop_go_pens)?;
        cursor.write_u8(self.grid_position)?;
        cursor.write_u8(self.driver_status)?;
        cursor.write_u8(self.result_status)?;
        cursor.write_u8(self.pit_lane_timer_active)?;
        cursor.write_u16::<LittleEndian>(self.pit_lane_time_in_lane_in_ms)?;
        cursor.write_u16::<LittleEndian>(self.pit_stop_timer_in_ms)?;
        cursor.write_u8(self.pit_stop_should_serve_pen)?;

        Ok(buffer)
    }
}

impl PacketLapData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(PacketLapData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            lap_data: {
                let mut lap_data: [LapData; 22] = [LapData::default(); 22];
                for i in 0..22 {
                    lap_data[i] = LapData::unserialize(
                        &bytes[size_of::<PacketHeader>() + i * size_of::<LapData>()
                            ..size_of::<PacketHeader>() + (i + 1) * size_of::<LapData>()],
                    )?;
                }
                lap_data
            },
            time_trial_pb_car_idx: cursor.read_u8()?,
            time_trial_rival_car_idx: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<PacketLapData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_all(&self.header.serialize()?)?;
        for lap_data in self.lap_data {
            cursor.write_all(&lap_data.serialize()?)?;
        }
        cursor.write_u8(self.time_trial_pb_car_idx)?;
        cursor.write_u8(self.time_trial_rival_car_idx)?;

        Ok(buffer)
    }
}
