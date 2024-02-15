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

    #[test]
    fn test_tyre_set_data_serialization_deserialization() {
        // Create some sample tyre set data
        let original_tyre_set_data: TyreSetData = TyreSetData {
            actual_tyre_compound: 1u8,
            visual_tyre_compound: 2u8,
            wear: 50u8,
            available: 1u8,
            recommended_session: 0u8,
            life_span: 80u8,
            usable_life: 60u8,
            lap_delta_time: 500i16,
            fitted: 1u8,
        };

        // Serialize the data
        let serialized_data: Vec<u8> = original_tyre_set_data.serialize().unwrap();

        // Deserialize the serialized data
        let deserialized_tyre_set_data: TyreSetData =
            TyreSetData::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(original_tyre_set_data, deserialized_tyre_set_data);
    }

    #[test]
    fn test_packet_tyre_sets_data_serialization_deserialization() {
        // Create some sample packet tyre sets data
        let mut original_packet_tyre_sets_data: PacketTyreSetsData = PacketTyreSetsData::default();
        original_packet_tyre_sets_data.header.packet_format = 2021u16;
        original_packet_tyre_sets_data.header.game_year = 21u8;
        original_packet_tyre_sets_data.header.game_major_version = 1u8;
        original_packet_tyre_sets_data.header.game_minor_version = 3u8;
        original_packet_tyre_sets_data.header.packet_version = 1u8;
        original_packet_tyre_sets_data.header.packet_id = 0u8;
        original_packet_tyre_sets_data.header.session_uid = 123456789u64;
        original_packet_tyre_sets_data.header.session_time = 123.456f32;
        original_packet_tyre_sets_data.header.frame_identifier = 1000u32;
        original_packet_tyre_sets_data
            .header
            .overall_frame_identifier = 5000u32;
        original_packet_tyre_sets_data.header.player_car_index = 1u8;
        original_packet_tyre_sets_data
            .header
            .secondary_player_car_index = 255u8;
        original_packet_tyre_sets_data.car_idx = 1u8;
        for i in 0..20 {
            original_packet_tyre_sets_data.tyre_set_data[i].actual_tyre_compound = (i + 1) as u8;
            original_packet_tyre_sets_data.tyre_set_data[i].visual_tyre_compound = (i + 2) as u8;
            original_packet_tyre_sets_data.tyre_set_data[i].wear = (i + 3) as u8;
            original_packet_tyre_sets_data.tyre_set_data[i].available = (i + 4) as u8;
            original_packet_tyre_sets_data.tyre_set_data[i].recommended_session = (i + 5) as u8;
            original_packet_tyre_sets_data.tyre_set_data[i].life_span = (i + 6) as u8;
            original_packet_tyre_sets_data.tyre_set_data[i].usable_life = (i + 7) as u8;
            original_packet_tyre_sets_data.tyre_set_data[i].lap_delta_time = (i + 8) as i16;
            original_packet_tyre_sets_data.tyre_set_data[i].fitted = (i + 9) as u8;
        }
        original_packet_tyre_sets_data.fitted_idx = 2;

        // Serialize the data
        let serialized_data: Vec<u8> = original_packet_tyre_sets_data.serialize().unwrap();

        // Deserialize the serialized data
        let deserialized_packet_tyre_sets_data: PacketTyreSetsData =
            PacketTyreSetsData::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(
            original_packet_tyre_sets_data,
            deserialized_packet_tyre_sets_data
        );
    }
}
