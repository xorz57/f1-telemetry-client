use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TyreStintHistoryData {
    pub end_lap: u8,              // 1 Byte
    pub tyre_actual_compound: u8, // 1 Byte
    pub tyre_visual_compound: u8, // 1 Byte
} // 3 Bytes

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl LapHistoryData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
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
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<LapHistoryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u32::<LittleEndian>(self.lap_time_in_ms)?;
        cursor.write_u16::<LittleEndian>(self.sector1_time_in_ms)?;
        cursor.write_u8(self.sector1_time_minutes)?;
        cursor.write_u16::<LittleEndian>(self.sector2_time_in_ms)?;
        cursor.write_u8(self.sector2_time_minutes)?;
        cursor.write_u16::<LittleEndian>(self.sector3_time_in_ms)?;
        cursor.write_u8(self.sector3_time_minutes)?;
        cursor.write_u8(self.lap_valid_bit_flags)?;

        Ok(bytes)
    }
}

impl TyreStintHistoryData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(TyreStintHistoryData {
            end_lap: cursor.read_u8()?,
            tyre_actual_compound: cursor.read_u8()?,
            tyre_visual_compound: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<TyreStintHistoryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.end_lap)?;
        cursor.write_u8(self.tyre_actual_compound)?;
        cursor.write_u8(self.tyre_visual_compound)?;

        Ok(bytes)
    }
}

impl PacketSessionHistoryData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&bytes[size_of::<PacketHeader>()..]);

        Ok(PacketSessionHistoryData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
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
                    lap_history_data[i] = LapHistoryData::unserialize(
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
                    tyre_stints_history_data[i] = TyreStintHistoryData::unserialize(
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
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketSessionHistoryData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        cursor.write_u8(self.car_idx)?;
        cursor.write_u8(self.num_laps)?;
        cursor.write_u8(self.num_tyre_stints)?;
        cursor.write_u8(self.best_lap_time_lap_num)?;
        cursor.write_u8(self.best_sector1_lap_num)?;
        cursor.write_u8(self.best_sector2_lap_num)?;
        cursor.write_u8(self.best_sector3_lap_num)?;
        for element in self.lap_history_data {
            cursor.write_all(&element.serialize()?)?;
        }
        for element in self.tyre_stints_history_data {
            cursor.write_all(&element.serialize()?)?;
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_lap_history_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: LapHistoryData = LapHistoryData {
            lap_time_in_ms: rng.gen(),
            sector1_time_in_ms: rng.gen(),
            sector1_time_minutes: rng.gen(),
            sector2_time_in_ms: rng.gen(),
            sector2_time_minutes: rng.gen(),
            sector3_time_in_ms: rng.gen(),
            sector3_time_minutes: rng.gen(),
            lap_valid_bit_flags: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: LapHistoryData =
            LapHistoryData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_tyre_stint_history_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: TyreStintHistoryData = TyreStintHistoryData {
            end_lap: rng.gen(),
            tyre_actual_compound: rng.gen(),
            tyre_visual_compound: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: TyreStintHistoryData =
            TyreStintHistoryData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_session_history_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketSessionHistoryData = PacketSessionHistoryData {
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
            lap_history_data: [LapHistoryData {
                lap_time_in_ms: rng.gen(),
                sector1_time_in_ms: rng.gen(),
                sector1_time_minutes: rng.gen(),
                sector2_time_in_ms: rng.gen(),
                sector2_time_minutes: rng.gen(),
                sector3_time_in_ms: rng.gen(),
                sector3_time_minutes: rng.gen(),
                lap_valid_bit_flags: rng.gen(),
            }; 100],
            car_idx: rng.gen(),
            num_laps: rng.gen(),
            num_tyre_stints: rng.gen(),
            best_lap_time_lap_num: rng.gen(),
            best_sector1_lap_num: rng.gen(),
            best_sector2_lap_num: rng.gen(),
            best_sector3_lap_num: rng.gen(),
            tyre_stints_history_data: [TyreStintHistoryData {
                end_lap: rng.gen(),
                tyre_actual_compound: rng.gen(),
                tyre_visual_compound: rng.gen(),
            }; 8],
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketSessionHistoryData =
            PacketSessionHistoryData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
