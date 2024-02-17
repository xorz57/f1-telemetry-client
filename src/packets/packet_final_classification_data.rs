use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PacketFinalClassificationData {
    pub header: PacketHeader,                               // 29 Bytes
    pub num_cars: u8,                                       // 1 Byte
    pub classification_data: [FinalClassificationData; 22], // 990 Bytes
} // 1020 Bytes

impl FinalClassificationData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
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
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<FinalClassificationData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

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
        for element in self.tyre_stints_actual {
            cursor.write_u8(element)?;
        }
        for element in self.tyre_stints_visual {
            cursor.write_u8(element)?;
        }
        for element in self.tyre_stints_end_laps {
            cursor.write_u8(element)?;
        }

        Ok(bytes)
    }
}

impl PacketFinalClassificationData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&bytes);

        Ok(PacketFinalClassificationData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            num_cars: {
                let pos: usize = size_of::<PacketHeader>();
                cursor.set_position(pos as u64);
                cursor.read_u8()?
            },
            classification_data: {
                let mut classification_data: [FinalClassificationData; 22] =
                    [FinalClassificationData::default(); 22];
                for i in 0..22 {
                    classification_data[i] = FinalClassificationData::unserialize(
                        &bytes[size_of::<PacketHeader>()
                            + 1 * size_of::<u8>()
                            + i * size_of::<FinalClassificationData>()
                            ..size_of::<PacketHeader>()
                                + 1 * size_of::<u8>()
                                + (i + 1) * size_of::<FinalClassificationData>()],
                    )?;
                }
                classification_data
            },
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<FinalClassificationData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        cursor.write_u8(self.num_cars)?;
        for element in self.classification_data {
            cursor.write_all(&element.serialize()?)?;
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_classification_data_serialization_deserialization() {
        let original_data: FinalClassificationData = FinalClassificationData {
            position: 1u8,
            num_laps: 2u8,
            grid_position: 3u8,
            points: 4u8,
            num_pit_stops: 5u8,
            result_status: 6u8,
            best_lap_time_in_ms: 100u32,
            total_race_time: 123.456f64,
            penalties_time: 7u8,
            num_penalties: 8u8,
            num_tyre_stints: 9u8,
            tyre_stints_actual: [1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8],
            tyre_stints_visual: [2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8],
            tyre_stints_end_laps: [3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8],
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: FinalClassificationData =
            FinalClassificationData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_final_classification_data_serialization_deserialization() {
        let original_packet: PacketFinalClassificationData = PacketFinalClassificationData {
            header: PacketHeader {
                packet_format: 2021u16,
                game_year: 21u8,
                game_major_version: 1u8,
                game_minor_version: 3u8,
                packet_version: 1u8,
                packet_id: 0u8,
                session_uid: 123456789u64,
                session_time: 123.456f32,
                frame_identifier: 1000u32,
                overall_frame_identifier: 5000u32,
                player_car_index: 1u8,
                secondary_player_car_index: 255u8,
            },
            num_cars: 22u8,
            classification_data: [FinalClassificationData {
                position: 1u8,
                num_laps: 2u8,
                grid_position: 3u8,
                points: 4u8,
                num_pit_stops: 5u8,
                result_status: 6u8,
                best_lap_time_in_ms: 100u32,
                total_race_time: 123.456f64,
                penalties_time: 7u8,
                num_penalties: 8u8,
                num_tyre_stints: 9u8,
                tyre_stints_actual: [1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8],
                tyre_stints_visual: [2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8],
                tyre_stints_end_laps: [3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8],
            }; 22],
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketFinalClassificationData =
            PacketFinalClassificationData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
