use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CarTelemetryData {
    pub speed: u16,                         // 2 Bytes
    pub throttle: f32,                      // 4 Bytes
    pub steer: f32,                         // 4 Bytes
    pub brake: f32,                         // 4 Bytes
    pub clutch: u8,                         // 1 Byte
    pub gear: i8,                           // 1 Byte
    pub engine_rpm: u16,                    // 2 Bytes
    pub drs: u8,                            // 1 Byte
    pub rev_lights_percent: u8,             // 1 Byte
    pub rev_lights_bit_value: u16,          // 2 Bytes
    pub brakes_temperature: [u16; 4],       // 8 Bytes
    pub tyres_surface_temperature: [u8; 4], // 4 Bytes
    pub tyres_inner_temperature: [u8; 4],   // 4 Bytes
    pub engine_temperature: u16,            // 2 Bytes
    pub tyres_pressure: [f32; 4],           // 16 Bytes
    pub surface_type: [u8; 4],              // 4 Bytes
} // 60 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PacketCarTelemetryData {
    pub header: PacketHeader,                       // 29 Bytes
    pub car_telemetry_data: [CarTelemetryData; 22], // 1320 Bytes
    pub mfd_panel_index: u8,                        // 1 Byte
    pub mfd_panel_index_secondary_player: u8,       // 1 Byte
    pub suggested_gear: i8,                         // 1 Byte
} // 1352 Bytes

impl CarTelemetryData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(CarTelemetryData {
            speed: cursor.read_u16::<LittleEndian>()?,
            throttle: cursor.read_f32::<LittleEndian>()?,
            steer: cursor.read_f32::<LittleEndian>()?,
            brake: cursor.read_f32::<LittleEndian>()?,
            clutch: cursor.read_u8()?,
            gear: cursor.read_i8()?,
            engine_rpm: cursor.read_u16::<LittleEndian>()?,
            drs: cursor.read_u8()?,
            rev_lights_percent: cursor.read_u8()?,
            rev_lights_bit_value: cursor.read_u16::<LittleEndian>()?,
            brakes_temperature: [
                cursor.read_u16::<LittleEndian>()?,
                cursor.read_u16::<LittleEndian>()?,
                cursor.read_u16::<LittleEndian>()?,
                cursor.read_u16::<LittleEndian>()?,
            ],
            tyres_surface_temperature: [
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
            ],
            tyres_inner_temperature: [
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
            ],
            engine_temperature: cursor.read_u16::<LittleEndian>()?,
            tyres_pressure: [
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ],
            surface_type: [
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
            ],
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<CarTelemetryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u16::<LittleEndian>(self.speed)?;
        cursor.write_f32::<LittleEndian>(self.throttle)?;
        cursor.write_f32::<LittleEndian>(self.steer)?;
        cursor.write_f32::<LittleEndian>(self.brake)?;
        cursor.write_u8(self.clutch)?;
        cursor.write_i8(self.gear)?;
        cursor.write_u16::<LittleEndian>(self.engine_rpm)?;
        cursor.write_u8(self.drs)?;
        cursor.write_u8(self.rev_lights_percent)?;
        cursor.write_u16::<LittleEndian>(self.rev_lights_bit_value)?;
        for element in self.brakes_temperature {
            cursor.write_u16::<LittleEndian>(element)?;
        }
        for element in self.tyres_surface_temperature {
            cursor.write_u8(element)?;
        }
        for element in self.tyres_inner_temperature {
            cursor.write_u8(element)?;
        }
        cursor.write_u16::<LittleEndian>(self.engine_temperature)?;
        for element in self.tyres_pressure {
            cursor.write_f32::<LittleEndian>(element)?;
        }
        for element in self.surface_type {
            cursor.write_u8(element)?;
        }

        Ok(bytes)
    }
}

impl PacketCarTelemetryData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&bytes);

        Ok(PacketCarTelemetryData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            car_telemetry_data: {
                let mut car_telemetry_data: [CarTelemetryData; 22] =
                    [CarTelemetryData::default(); 22];
                for i in 0..22 {
                    car_telemetry_data[i] = CarTelemetryData::unserialize(
                        &bytes[size_of::<PacketHeader>() + i * size_of::<CarTelemetryData>()
                            ..size_of::<PacketHeader>() + (i + 1) * size_of::<CarTelemetryData>()],
                    )?;
                }
                car_telemetry_data
            },
            mfd_panel_index: {
                let pos: usize = size_of::<PacketHeader>() + size_of::<[CarTelemetryData; 22]>();
                cursor.set_position(pos as u64);
                cursor.read_u8()?
            },
            mfd_panel_index_secondary_player: cursor.read_u8()?,
            suggested_gear: cursor.read_i8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketCarTelemetryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        for element in self.car_telemetry_data {
            cursor.write_all(&element.serialize()?)?;
        }
        cursor.write_u8(self.mfd_panel_index)?;
        cursor.write_u8(self.mfd_panel_index_secondary_player)?;
        cursor.write_i8(self.suggested_gear)?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_telemetry_data_serialization_deserialization() {
        let original_data: CarTelemetryData = CarTelemetryData {
            speed: 200u16,
            throttle: 0.8f32,
            steer: 0.2f32,
            brake: 0.0f32,
            clutch: 0u8,
            gear: 3i8,
            engine_rpm: 8000u16,
            drs: 1u8,
            rev_lights_percent: 50u8,
            rev_lights_bit_value: 1000u16,
            brakes_temperature: [500u16, 550u16, 600u16, 625u16],
            tyres_surface_temperature: [90u8, 91u8, 92u8, 93u8],
            tyres_inner_temperature: [85u8, 86u8, 87u8, 88u8],
            engine_temperature: 95u16,
            tyres_pressure: [1.9f32, 1.8f32, 1.9f32, 2.0f32],
            surface_type: [1u8, 2u8, 3u8, 4u8],
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: CarTelemetryData =
            CarTelemetryData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_car_telemetry_data_serialization_deserialization() {
        let mut original_packet: PacketCarTelemetryData = PacketCarTelemetryData::default();
        original_packet.header.packet_format = 2021u16;
        original_packet.header.game_year = 21u8;
        original_packet.header.game_major_version = 1u8;
        original_packet.header.game_minor_version = 3u8;
        original_packet.header.packet_version = 1u8;
        original_packet.header.packet_id = 0u8;
        original_packet.header.session_uid = 123456789u64;
        original_packet.header.session_time = 123.456f32;
        original_packet.header.frame_identifier = 1000u32;
        original_packet.header.overall_frame_identifier = 5000u32;
        original_packet.header.player_car_index = 1u8;
        original_packet.header.secondary_player_car_index = 255u8;
        for i in 0..22 {
            original_packet.car_telemetry_data[i].speed = (i * 10) as u16;
            original_packet.car_telemetry_data[i].throttle = (i as f32) * 0.1;
            original_packet.car_telemetry_data[i].steer = (i as f32) * 0.01;
            original_packet.car_telemetry_data[i].brake = (i as f32) * 0.02;
            original_packet.car_telemetry_data[i].clutch = i as u8;
            original_packet.car_telemetry_data[i].gear = i as i8 - 10;
            original_packet.car_telemetry_data[i].engine_rpm = (i * 1000) as u16;
            original_packet.car_telemetry_data[i].drs = i as u8 % 2;
            original_packet.car_telemetry_data[i].rev_lights_percent = (i * 5) as u8;
            original_packet.car_telemetry_data[i].rev_lights_bit_value = (i * 100) as u16;
            for j in 0..4 {
                original_packet.car_telemetry_data[i].brakes_temperature[j] =
                    (i * 100 + j * 10) as u16;
                original_packet.car_telemetry_data[i].tyres_surface_temperature[j] =
                    (i * 10 + j) as u8;
                original_packet.car_telemetry_data[i].tyres_inner_temperature[j] =
                    (i * 5 + j) as u8;
                original_packet.car_telemetry_data[i].tyres_pressure[j] = (i * 1 + j) as f32;
                original_packet.car_telemetry_data[i].surface_type[j] = (i * 10 + j) as u8;
            }
        }
        original_packet.mfd_panel_index = 3u8;
        original_packet.mfd_panel_index_secondary_player = 2u8;
        original_packet.suggested_gear = -1i8;

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketCarTelemetryData =
            PacketCarTelemetryData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
