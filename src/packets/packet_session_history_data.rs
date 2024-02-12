use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LapHistoryData {
    pub lap_time_in_ms: u32,      // 4 Bytes
    pub sector1_time_in_ms: u16,  // 2 Bytes
    pub sector1_time_minutes: u8, // 1 Byte
    pub sector2_time_in_ms: u16,  // 2 Bytes
    pub sector2_time_minutes: u8, // 1 Byte
    pub sector3_time_in_ms: u16,  // 2 Bytes
    pub sector3_time_minutes: u8, // 1 Byte
    pub lap_valid_bit_flags: u8,  // 1 Byte
} // 14 Bytes

impl LapHistoryData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(LapHistoryData {
            lap_time_in_ms: cursor.read_u32::<LittleEndian>()?,
            sector1_time_in_ms: cursor.read_u16::<LittleEndian>()?,
            sector1_time_minutes: cursor.read_u8()?,
            sector2_time_in_ms: cursor.read_u16::<LittleEndian>()?,
            sector2_time_minutes: cursor.read_u8()?,
            sector3_time_in_ms: cursor.read_u16::<LittleEndian>()?,
            sector3_time_minutes: cursor.read_u8()?,
            lap_valid_bit_flags: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<LapHistoryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_u32::<LittleEndian>(self.lap_time_in_ms)?;
        cursor.write_u16::<LittleEndian>(self.sector1_time_in_ms)?;
        cursor.write_u8(self.sector1_time_minutes)?;
        cursor.write_u16::<LittleEndian>(self.sector2_time_in_ms)?;
        cursor.write_u8(self.sector2_time_minutes)?;
        cursor.write_u16::<LittleEndian>(self.sector3_time_in_ms)?;
        cursor.write_u8(self.sector3_time_minutes)?;
        cursor.write_u8(self.lap_valid_bit_flags)?;

        Ok(buffer)
    }
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TyreStintHistoryData {
    pub end_lap: u8,              // 1 Byte
    pub tyre_actual_compound: u8, // 1 Byte
    pub tyre_visual_compound: u8, // 1 Byte
} // 3 Bytes

impl TyreStintHistoryData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(TyreStintHistoryData {
            end_lap: cursor.read_u8()?,
            tyre_actual_compound: cursor.read_u8()?,
            tyre_visual_compound: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<TyreStintHistoryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_u8(self.end_lap)?;
        cursor.write_u8(self.tyre_actual_compound)?;
        cursor.write_u8(self.tyre_visual_compound)?;

        Ok(buffer)
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketSessionHistoryData {
    pub header: PacketHeader,                                // 29 Bytes
    pub car_idx: u8,                                         // 1 Byte
    pub num_laps: u8,                                        // 1 Byte
    pub num_tyre_stints: u8,                                 // 1 Byte
    pub best_lap_time_lap_num: u8,                           // 1 Byte
    pub best_sector1_lap_num: u8,                            // 1 Byte
    pub best_sector2_lap_num: u8,                            // 1 Byte
    pub best_sector3_lap_num: u8,                            // 1 Byte
    pub lap_history_data: [LapHistoryData; 100],             // 1400 Bytes
    pub tyre_stints_history_data: [TyreStintHistoryData; 8], // 24 Bytes
} // 1460 Bytes

impl Default for PacketSessionHistoryData {
    fn default() -> Self {
        PacketSessionHistoryData {
            header: PacketHeader::default(),
            car_idx: 0u8,
            num_laps: 0u8,
            num_tyre_stints: 0u8,
            best_lap_time_lap_num: 0u8,
            best_sector1_lap_num: 0u8,
            best_sector2_lap_num: 0u8,
            best_sector3_lap_num: 0u8,
            lap_history_data: [LapHistoryData::default(); 100],
            tyre_stints_history_data: [TyreStintHistoryData::default(); 8],
        }
    }
}

impl PacketSessionHistoryData {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&bytes[size_of::<PacketHeader>()..]);

        Ok(PacketSessionHistoryData {
            header: PacketHeader::from_bytes(&bytes[..size_of::<PacketHeader>()])?,
            car_idx: cursor.read_u8()?,
            num_laps: cursor.read_u8()?,
            num_tyre_stints: cursor.read_u8()?,
            best_lap_time_lap_num: cursor.read_u8()?,
            best_sector1_lap_num: cursor.read_u8()?,
            best_sector2_lap_num: cursor.read_u8()?,
            best_sector3_lap_num: cursor.read_u8()?,
            lap_history_data: {
                let mut lap_history_data: [LapHistoryData; 100] = [LapHistoryData::default(); 100];
                for i in 0..100 {
                    lap_history_data[i] = LapHistoryData::from_bytes(
                        &bytes[size_of::<PacketHeader>()
                            + 7 * size_of::<u8>()
                            + i * size_of::<LapHistoryData>()
                            ..size_of::<PacketHeader>()
                                + 7 * size_of::<u8>()
                                + (i + 1) * size_of::<LapHistoryData>()],
                    )?;
                }
                lap_history_data
            },
            tyre_stints_history_data: {
                let mut tyre_stints_history_data: [TyreStintHistoryData; 8] =
                    [TyreStintHistoryData::default(); 8];
                for i in 0..8 {
                    tyre_stints_history_data[i] = TyreStintHistoryData::from_bytes(
                        &bytes[size_of::<[LapHistoryData; 100]>()
                            + size_of::<PacketHeader>()
                            + 7 * size_of::<u8>()
                            + i * size_of::<TyreStintHistoryData>()
                            ..size_of::<[LapHistoryData; 100]>()
                                + size_of::<PacketHeader>()
                                + 7 * size_of::<u8>()
                                + (i + 1) * size_of::<TyreStintHistoryData>()],
                    )?;
                }
                tyre_stints_history_data
            },
        })
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<PacketSessionHistoryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buffer);

        cursor.write_all(&self.header.to_bytes()?)?;
        cursor.write_u8(self.car_idx)?;
        cursor.write_u8(self.num_laps)?;
        cursor.write_u8(self.num_tyre_stints)?;
        cursor.write_u8(self.best_lap_time_lap_num)?;
        cursor.write_u8(self.best_sector1_lap_num)?;
        cursor.write_u8(self.best_sector2_lap_num)?;
        cursor.write_u8(self.best_sector3_lap_num)?;
        for lap_history_data in self.lap_history_data {
            cursor.write_all(&lap_history_data.to_bytes()?)?;
        }
        for tyre_stint_history_data in self.tyre_stints_history_data {
            cursor.write_all(&tyre_stint_history_data.to_bytes()?)?;
        }

        Ok(buffer)
    }
}
