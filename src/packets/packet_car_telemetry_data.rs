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
        // Create some sample car telemetry data
        let original_car_telemetry_data: CarTelemetryData = CarTelemetryData {
            speed: 200,
            throttle: 0.8,
            steer: 0.2,
            brake: 0.0,
            clutch: 0,
            gear: 3,
            engine_rpm: 8000,
            drs: 1,
            rev_lights_percent: 50,
            rev_lights_bit_value: 1000,
            brakes_temperature: [500, 550, 600, 625],
            tyres_surface_temperature: [90, 91, 92, 93],
            tyres_inner_temperature: [85, 86, 87, 88],
            engine_temperature: 95,
            tyres_pressure: [1.9, 1.8, 1.9, 2.0],
            surface_type: [1, 2, 3, 4],
        };

        // Serialize the data
        let serialized_data: Vec<u8> = original_car_telemetry_data.serialize().unwrap();

        // Deserialize the serialized data
        let deserialized_car_telemetry_data: CarTelemetryData =
            CarTelemetryData::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(original_car_telemetry_data, deserialized_car_telemetry_data);
    }

    #[test]
    fn test_packet_car_telemetry_data_serialization_deserialization() {
        // Create some sample packet car telemetry data
        let mut original_packet_car_telemetry_data: PacketCarTelemetryData =
            PacketCarTelemetryData::default();
        original_packet_car_telemetry_data.header.packet_format = 2021;
        original_packet_car_telemetry_data.header.game_year = 21;
        original_packet_car_telemetry_data.header.game_major_version = 1;
        original_packet_car_telemetry_data.header.game_minor_version = 3;
        original_packet_car_telemetry_data.header.packet_version = 1;
        original_packet_car_telemetry_data.header.packet_id = 0;
        original_packet_car_telemetry_data.header.session_uid = 123456789;
        original_packet_car_telemetry_data.header.session_time = 123.456;
        original_packet_car_telemetry_data.header.frame_identifier = 1000;
        original_packet_car_telemetry_data
            .header
            .overall_frame_identifier = 5000;
        original_packet_car_telemetry_data.header.player_car_index = 1;
        original_packet_car_telemetry_data
            .header
            .secondary_player_car_index = 255;

        // Populate car telemetry data array with some sample data
        for i in 0..22 {
            original_packet_car_telemetry_data.car_telemetry_data[i].speed = (i * 10) as u16;
            original_packet_car_telemetry_data.car_telemetry_data[i].throttle = (i as f32) * 0.1;
            original_packet_car_telemetry_data.car_telemetry_data[i].steer = (i as f32) * 0.01;
            original_packet_car_telemetry_data.car_telemetry_data[i].brake = (i as f32) * 0.02;
            original_packet_car_telemetry_data.car_telemetry_data[i].clutch = i as u8;
            original_packet_car_telemetry_data.car_telemetry_data[i].gear = i as i8 - 10;
            original_packet_car_telemetry_data.car_telemetry_data[i].engine_rpm = (i * 1000) as u16;
            original_packet_car_telemetry_data.car_telemetry_data[i].drs = i as u8 % 2;
            original_packet_car_telemetry_data.car_telemetry_data[i].rev_lights_percent =
                (i * 5) as u8;
            original_packet_car_telemetry_data.car_telemetry_data[i].rev_lights_bit_value =
                (i * 100) as u16;
            for j in 0..4 {
                original_packet_car_telemetry_data.car_telemetry_data[i].brakes_temperature[j] =
                    (i * 100 + j * 10) as u16;
                original_packet_car_telemetry_data.car_telemetry_data[i]
                    .tyres_surface_temperature[j] = (i * 10 + j) as u8;
                original_packet_car_telemetry_data.car_telemetry_data[i].tyres_inner_temperature
                    [j] = (i * 5 + j) as u8;
                original_packet_car_telemetry_data.car_telemetry_data[i].tyres_pressure[j] =
                    (i * 1 + j) as f32;
                original_packet_car_telemetry_data.car_telemetry_data[i].surface_type[j] =
                    (i * 10 + j) as u8;
            }
        }

        // Set other fields
        original_packet_car_telemetry_data.mfd_panel_index = 3;
        original_packet_car_telemetry_data.mfd_panel_index_secondary_player = 2;
        original_packet_car_telemetry_data.suggested_gear = -1;

        // Serialize the data
        let serialized_data: Vec<u8> = original_packet_car_telemetry_data.serialize().unwrap();

        // Deserialize the serialized data
        let deserialized_packet_car_telemetry_data: PacketCarTelemetryData =
            PacketCarTelemetryData::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(
            original_packet_car_telemetry_data,
            deserialized_packet_car_telemetry_data
        );
    }
}
