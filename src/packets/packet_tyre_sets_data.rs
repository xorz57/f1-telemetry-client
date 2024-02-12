use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

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
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
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
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<TyreSetData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.actual_tyre_compound)?;
        cursor.write_u8(self.visual_tyre_compound)?;
        cursor.write_u8(self.wear)?;
        cursor.write_u8(self.available)?;
        cursor.write_u8(self.recommended_session)?;
        cursor.write_u8(self.life_span)?;
        cursor.write_u8(self.usable_life)?;
        cursor.write_i16::<LittleEndian>(self.lap_delta_time)?;
        cursor.write_u8(self.fitted)?;

        Ok(bytes)
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
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(PacketTyreSetsData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            car_idx: cursor.read_u8()?,
            tyre_set_data: {
                let mut tyre_set_data: [TyreSetData; 20] = [TyreSetData::default(); 20];
                for i in 0..20 {
                    tyre_set_data[i] = TyreSetData::unserialize(
                        &bytes[size_of::<PacketHeader>() + i * size_of::<TyreSetData>()
                            ..size_of::<PacketHeader>() + (i + 1) * size_of::<TyreSetData>()],
                    )?;
                }
                tyre_set_data
            },
            fitted_idx: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketTyreSetsData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        cursor.write_u8(self.car_idx)?;
        for tyre_set_data in self.tyre_set_data {
            cursor.write_all(&tyre_set_data.serialize()?)?;
        }
        cursor.write_u8(self.fitted_idx)?;

        Ok(bytes)
    }
}
