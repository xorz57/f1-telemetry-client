use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FinalClassificationData {
    pub position: u8,                  // 1 Byte
    pub num_laps: u8,                  // 1 Byte
    pub grid_position: u8,             // 1 Byte
    pub points: u8,                    // 1 Byte
    pub num_pit_stops: u8,             // 1 Byte
    pub result_status: u8,             // 1 Byte
    pub best_lap_time_in_ms: u32,      // 4 Bytes
    pub total_race_time: f64,          // 8 Bytes
    pub penalties_time: u8,            // 1 Byte
    pub num_penalties: u8,             // 1 Byte
    pub num_tyre_stints: u8,           // 1 Byte
    pub tyre_stints_actual: [u8; 8],   // 8 Bytes
    pub tyre_stints_visual: [u8; 8],   // 8 Bytes
    pub tyre_stints_end_laps: [u8; 8], // 8 Bytes
} // 45 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketFinalClassificationData {
    pub header: PacketHeader,                               // 29 Bytes
    pub num_cars: u8,                                       // 1 Byte
    pub classification_data: [FinalClassificationData; 22], // 990 Bytes
} // 1020 Bytes

impl FinalClassificationData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(FinalClassificationData {
            position: cursor.read_u8()?,
            num_laps: cursor.read_u8()?,
            grid_position: cursor.read_u8()?,
            points: cursor.read_u8()?,
            num_pit_stops: cursor.read_u8()?,
            result_status: cursor.read_u8()?,
            best_lap_time_in_ms: cursor.read_u32::<LittleEndian>()?,
            total_race_time: cursor.read_f64::<LittleEndian>()?,
            penalties_time: cursor.read_u8()?,
            num_penalties: cursor.read_u8()?,
            num_tyre_stints: cursor.read_u8()?,
            tyre_stints_actual: {
                let mut tyre_stints_actual: [u8; 8] = [0u8; 8];
                for i in 0..8 {
                    tyre_stints_actual[i] = cursor.read_u8()?;
                }
                tyre_stints_actual
            },
            tyre_stints_visual: {
                let mut tyre_stints_visual: [u8; 8] = [0u8; 8];
                for i in 0..8 {
                    tyre_stints_visual[i] = cursor.read_u8()?;
                }
                tyre_stints_visual
            },
            tyre_stints_end_laps: {
                let mut tyre_stints_end_laps: [u8; 8] = [0u8; 8];
                for i in 0..8 {
                    tyre_stints_end_laps[i] = cursor.read_u8()?;
                }
                tyre_stints_end_laps
            },
        })
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<FinalClassificationData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_u8(self.position)?;
        cursor.write_u8(self.num_laps)?;
        cursor.write_u8(self.grid_position)?;
        cursor.write_u8(self.points)?;
        cursor.write_u8(self.num_pit_stops)?;
        cursor.write_u8(self.result_status)?;
        cursor.write_u32::<LittleEndian>(self.best_lap_time_in_ms)?;
        cursor.write_f64::<LittleEndian>(self.total_race_time)?;
        cursor.write_u8(self.penalties_time)?;
        cursor.write_u8(self.num_penalties)?;
        cursor.write_u8(self.num_tyre_stints)?;
        for tyre_stints_actual in self.tyre_stints_actual {
            cursor.write_u8(tyre_stints_actual)?;
        }
        for tyre_stints_visual in self.tyre_stints_visual {
            cursor.write_u8(tyre_stints_visual)?;
        }
        for tyre_stints_end_laps in self.tyre_stints_end_laps {
            cursor.write_u8(tyre_stints_end_laps)?;
        }

        Ok(buffer)
    }
}

impl PacketFinalClassificationData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&bytes[..size_of::<PacketHeader>()]);

        Ok(PacketFinalClassificationData {
            header: PacketHeader::from_bytes(&bytes[..size_of::<PacketHeader>()])?,
            num_cars: cursor.read_u8()?,
            classification_data: {
                let mut classification_data: [FinalClassificationData; 22] =
                    [FinalClassificationData::default(); 22];
                for i in 0..22 {
                    classification_data[i] = FinalClassificationData::from_bytes(
                        &bytes[1 * size_of::<u8>() + i * size_of::<FinalClassificationData>()
                            ..1 * size_of::<u8>() + (i + 1) * size_of::<FinalClassificationData>()],
                    )?;
                }
                classification_data
            },
        })
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<FinalClassificationData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_all(&self.header.to_bytes()?)?;
        cursor.write_u8(self.num_cars)?;
        for classification_data in self.classification_data {
            cursor.write_all(&classification_data.to_bytes()?)?;
        }

        Ok(buffer)
    }
}
