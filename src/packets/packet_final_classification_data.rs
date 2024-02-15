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
            header: {
                let pos: usize = size_of::<PacketHeader>();
                cursor.set_position(pos as u64);
                PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?
            },
            num_cars: cursor.read_u8()?,
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
    fn test_packet_final_classification_data_serialization_deserialization() {
        // Create some sample packet final classification data
        let mut original_packet_final_classification_data: PacketFinalClassificationData =
            PacketFinalClassificationData::default();
        original_packet_final_classification_data
            .header
            .packet_format = 2021;
        original_packet_final_classification_data.header.game_year = 21;
        original_packet_final_classification_data
            .header
            .game_major_version = 1;
        original_packet_final_classification_data
            .header
            .game_minor_version = 3;
        original_packet_final_classification_data
            .header
            .packet_version = 1;
        original_packet_final_classification_data.header.packet_id = 0;
        original_packet_final_classification_data.header.session_uid = 123456789;
        original_packet_final_classification_data
            .header
            .session_time = 123.456;
        original_packet_final_classification_data
            .header
            .frame_identifier = 1000;
        original_packet_final_classification_data
            .header
            .overall_frame_identifier = 5000;
        original_packet_final_classification_data
            .header
            .player_car_index = 1;
        original_packet_final_classification_data
            .header
            .secondary_player_car_index = 255;
        original_packet_final_classification_data.num_cars = 22;
        for i in 0..22 {
            original_packet_final_classification_data.classification_data[i].position =
                (i + 1) as u8;
            original_packet_final_classification_data.classification_data[i].num_laps =
                (i + 1) as u8;
            original_packet_final_classification_data.classification_data[i].grid_position =
                (i + 1) as u8;
            original_packet_final_classification_data.classification_data[i].points = (i + 1) as u8;
            original_packet_final_classification_data.classification_data[i].num_pit_stops =
                (i + 1) as u8;
            original_packet_final_classification_data.classification_data[i].result_status =
                (i + 1) as u8;
            original_packet_final_classification_data.classification_data[i].best_lap_time_in_ms =
                (i + 1) as u32;
            original_packet_final_classification_data.classification_data[i].total_race_time =
                (i + 1) as f64;
            original_packet_final_classification_data.classification_data[i].penalties_time =
                (i + 1) as u8;
            original_packet_final_classification_data.classification_data[i].num_penalties =
                (i + 1) as u8;
            original_packet_final_classification_data.classification_data[i].num_tyre_stints =
                (i + 1) as u8;
            for j in 0..8 {
                original_packet_final_classification_data.classification_data[i]
                    .tyre_stints_actual[j] = (i + 1) as u8;
                original_packet_final_classification_data.classification_data[i]
                    .tyre_stints_visual[j] = (i + 1) as u8;
                original_packet_final_classification_data.classification_data[i]
                    .tyre_stints_end_laps[j] = (i + 1) as u8;
            }
        }

        // Serialize the data
        let serialized_data: Vec<u8> = original_packet_final_classification_data
            .serialize()
            .unwrap();

        // Deserialize the serialized data
        let deserialized_packet_final_classification_data: PacketFinalClassificationData =
            PacketFinalClassificationData::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(
            original_packet_final_classification_data,
            deserialized_packet_final_classification_data
        );
    }
}
