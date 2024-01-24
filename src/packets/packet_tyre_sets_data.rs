use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TyreSetData {
    pub actual_tyre_compound: u8, // 1 Byte
    pub visual_tyre_compound: u8, // 1 Byte
    pub wear: u8,                 // 1 Byte
    pub available: u8,            // 1 Byte
    pub recommended_session: u8,  // 1 Byte
    pub life_span: u8,            // 1 Byte
    pub usable_life: u8,          // 1 Byte
    pub lap_delta_time: i16,      // 2 Bytes
    pub fitted: u8,               // 1 Byte
} // 10 Bytes

impl TyreSetData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(TyreSetData {
            actual_tyre_compound: cursor.read_u8()?,
            visual_tyre_compound: cursor.read_u8()?,
            wear: cursor.read_u8()?,
            available: cursor.read_u8()?,
            recommended_session: cursor.read_u8()?,
            life_span: cursor.read_u8()?,
            usable_life: cursor.read_u8()?,
            lap_delta_time: cursor.read_i16::<LittleEndian>()?,
            fitted: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(10);
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_u8(self.actual_tyre_compound)?;
        cursor.write_u8(self.visual_tyre_compound)?;
        cursor.write_u8(self.wear)?;
        cursor.write_u8(self.available)?;
        cursor.write_u8(self.recommended_session)?;
        cursor.write_u8(self.life_span)?;
        cursor.write_u8(self.usable_life)?;
        cursor.write_i16::<LittleEndian>(self.lap_delta_time)?;
        cursor.write_u8(self.fitted)?;

        Ok(buffer)
    }
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketTyreSetsData {
    pub header: PacketHeader,             // 29 Bytes
    pub car_idx: u8,                      // 1 Byte
    pub tyre_set_data: [TyreSetData; 20], // 200 Bytes
    pub fitted_idx: u8,                   // 1 Byte
} // 231 Bytes

impl PacketTyreSetsData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(PacketTyreSetsData {
            header: PacketHeader::from_bytes(&bytes[..29])?,
            car_idx: cursor.read_u8()?,
            tyre_set_data: {
                let mut tyre_set_data: [TyreSetData; 20] = [TyreSetData::default(); 20];
                for i in 0..20 {
                    tyre_set_data[i] =
                        TyreSetData::from_bytes(&bytes[30 + i * 10..30 + (i + 1) * 10])?;
                }
                tyre_set_data
            },
            fitted_idx: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(231);
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_all(&self.header.to_bytes()?)?;
        cursor.write_u8(self.car_idx)?;

        for tyre_set_data in &self.tyre_set_data {
            cursor.write_all(&tyre_set_data.to_bytes()?)?;
        }

        cursor.write_u8(self.fitted_idx)?;

        Ok(buffer)
    }
}