use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
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

impl CarTelemetryData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
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
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<CarTelemetryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

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

        Ok(buffer)
    }
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketCarTelemetryData {
    pub header: PacketHeader,                       // 29 Bytes
    pub car_telemetry_data: [CarTelemetryData; 22], // 1320 Bytes
    pub mfd_panel_index: u8,                        // 1 Byte
    pub mfd_panel_index_secondary_player: u8,       // 1 Byte
    pub suggested_gear: i8,                         // 1 Byte
} // 1352 Bytes

impl PacketCarTelemetryData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> =
            Cursor::new(&bytes[size_of::<PacketHeader>() + size_of::<[CarTelemetryData; 22]>()..]);

        Ok(PacketCarTelemetryData {
            header: PacketHeader::from_bytes(&bytes[..size_of::<PacketHeader>()])?,
            car_telemetry_data: {
                let mut car_telemetry_data: [CarTelemetryData; 22] =
                    [CarTelemetryData::default(); 22];
                for i in 0..22 {
                    car_telemetry_data[i] = CarTelemetryData::from_bytes(
                        &bytes[size_of::<PacketHeader>() + i * size_of::<CarTelemetryData>()
                            ..size_of::<PacketHeader>() + (i + 1) * size_of::<CarTelemetryData>()],
                    )?;
                }
                car_telemetry_data
            },
            mfd_panel_index: cursor.read_u8()?,
            mfd_panel_index_secondary_player: cursor.read_u8()?,
            suggested_gear: cursor.read_i8()?,
        })
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<PacketCarTelemetryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_all(&self.header.to_bytes()?)?;
        for car_telemetry_data in self.car_telemetry_data {
            cursor.write_all(&car_telemetry_data.to_bytes()?)?;
        }
        cursor.write_u8(self.mfd_panel_index)?;
        cursor.write_u8(self.mfd_panel_index_secondary_player)?;
        cursor.write_i8(self.suggested_gear)?;

        Ok(buffer)
    }
}
