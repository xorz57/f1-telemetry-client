use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PacketTyreSetsData {
    pub header: PacketHeader,             // 29 Bytes
    pub car_idx: u8,                      // 1 Byte
    pub tyre_set_data: [TyreSetData; 20], // 200 Bytes
    pub fitted_idx: u8,                   // 1 Byte
} // 231 Bytes

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

impl PacketTyreSetsData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(PacketTyreSetsData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            car_idx: {
                let pos: usize = size_of::<PacketHeader>();
                cursor.set_position(pos as u64);
                cursor.read_u8()?
            },
            tyre_set_data: {
                let mut tyre_set_data: [TyreSetData; 20] = [TyreSetData::default(); 20];
                for i in 0..20 {
                    tyre_set_data[i] = TyreSetData::unserialize(
                        &bytes[size_of::<PacketHeader>()
                            + 1 * size_of::<u8>()
                            + i * size_of::<TyreSetData>()
                            ..size_of::<PacketHeader>()
                                + 1 * size_of::<u8>()
                                + (i + 1) * size_of::<TyreSetData>()],
                    )?;
                }
                tyre_set_data
            },
            fitted_idx: {
                let pos: usize = size_of::<PacketHeader>()
                    + 1 * size_of::<u8>()
                    + size_of::<[TyreSetData; 20]>();
                cursor.set_position(pos as u64);
                cursor.read_u8()?
            },
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketTyreSetsData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        cursor.write_u8(self.car_idx)?;
        for element in self.tyre_set_data {
            cursor.write_all(&element.serialize()?)?;
        }
        cursor.write_u8(self.fitted_idx)?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_tyre_set_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: TyreSetData = TyreSetData {
            actual_tyre_compound: rng.gen(),
            visual_tyre_compound: rng.gen(),
            wear: rng.gen(),
            available: rng.gen(),
            recommended_session: rng.gen(),
            life_span: rng.gen(),
            usable_life: rng.gen(),
            lap_delta_time: rng.gen(),
            fitted: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: TyreSetData = TyreSetData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_tyre_sets_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketTyreSetsData = PacketTyreSetsData {
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
            car_idx: rng.gen(),
            tyre_set_data: [TyreSetData {
                actual_tyre_compound: rng.gen(),
                visual_tyre_compound: rng.gen(),
                wear: rng.gen(),
                available: rng.gen(),
                recommended_session: rng.gen(),
                life_span: rng.gen(),
                usable_life: rng.gen(),
                lap_delta_time: rng.gen(),
                fitted: rng.gen(),
            }; 20],
            fitted_idx: rng.gen(),
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketTyreSetsData =
            PacketTyreSetsData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
