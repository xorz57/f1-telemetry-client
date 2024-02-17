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
    use rand::Rng;

    #[test]
    fn test_car_telemetry_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: CarTelemetryData = CarTelemetryData {
            speed: rng.gen(),
            throttle: rng.gen(),
            steer: rng.gen(),
            brake: rng.gen(),
            clutch: rng.gen(),
            gear: rng.gen(),
            engine_rpm: rng.gen(),
            drs: rng.gen(),
            rev_lights_percent: rng.gen(),
            rev_lights_bit_value: rng.gen(),
            brakes_temperature: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            tyres_surface_temperature: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            tyres_inner_temperature: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            engine_temperature: 95u16,
            tyres_pressure: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            surface_type: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: CarTelemetryData =
            CarTelemetryData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_car_telemetry_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketCarTelemetryData = PacketCarTelemetryData {
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
            car_telemetry_data: [CarTelemetryData {
                speed: rng.gen(),
                throttle: rng.gen(),
                steer: rng.gen(),
                brake: rng.gen(),
                clutch: rng.gen(),
                gear: rng.gen(),
                engine_rpm: rng.gen(),
                drs: rng.gen(),
                rev_lights_percent: rng.gen(),
                rev_lights_bit_value: rng.gen(),
                brakes_temperature: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
                tyres_surface_temperature: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
                tyres_inner_temperature: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
                engine_temperature: 95u16,
                tyres_pressure: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
                surface_type: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
            }; 22],
            mfd_panel_index: rng.gen(),
            mfd_panel_index_secondary_player: rng.gen(),
            suggested_gear: rng.gen(),
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketCarTelemetryData =
            PacketCarTelemetryData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
