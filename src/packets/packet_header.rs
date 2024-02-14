use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PacketHeader {
    pub packet_format: u16,             // 2 Bytes
    pub game_year: u8,                  // 1 Byte
    pub game_major_version: u8,         // 1 Byte
    pub game_minor_version: u8,         // 1 Byte
    pub packet_version: u8,             // 1 Byte
    pub packet_id: u8,                  // 1 Byte
    pub session_uid: u64,               // 8 Bytes
    pub session_time: f32,              // 4 Bytes
    pub frame_identifier: u32,          // 4 Bytes
    pub overall_frame_identifier: u32,  // 4 Bytes
    pub player_car_index: u8,           // 1 Byte
    pub secondary_player_car_index: u8, // 1 Byte
} // 29 Bytes

impl PacketHeader {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(PacketHeader {
            packet_format: cursor.read_u16::<LittleEndian>()?,
            game_year: cursor.read_u8()?,
            game_major_version: cursor.read_u8()?,
            game_minor_version: cursor.read_u8()?,
            packet_version: cursor.read_u8()?,
            packet_id: cursor.read_u8()?,
            session_uid: cursor.read_u64::<LittleEndian>()?,
            session_time: cursor.read_f32::<LittleEndian>()?,
            frame_identifier: cursor.read_u32::<LittleEndian>()?,
            overall_frame_identifier: cursor.read_u32::<LittleEndian>()?,
            player_car_index: cursor.read_u8()?,
            secondary_player_car_index: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketHeader>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u16::<LittleEndian>(self.packet_format)?;
        cursor.write_u8(self.game_year)?;
        cursor.write_u8(self.game_major_version)?;
        cursor.write_u8(self.game_minor_version)?;
        cursor.write_u8(self.packet_version)?;
        cursor.write_u8(self.packet_id)?;
        cursor.write_u64::<LittleEndian>(self.session_uid)?;
        cursor.write_f32::<LittleEndian>(self.session_time)?;
        cursor.write_u32::<LittleEndian>(self.frame_identifier)?;
        cursor.write_u32::<LittleEndian>(self.overall_frame_identifier)?;
        cursor.write_u8(self.player_car_index)?;
        cursor.write_u8(self.secondary_player_car_index)?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_header_serialization_deserialization() {
        // Create some sample packet header data
        let original_packet_header: PacketHeader = PacketHeader {
            packet_format: 2021,
            game_year: 21,
            game_major_version: 1,
            game_minor_version: 3,
            packet_version: 1,
            packet_id: 0,
            session_uid: 123456789,
            session_time: 123.456,
            frame_identifier: 1000,
            overall_frame_identifier: 5000,
            player_car_index: 1,
            secondary_player_car_index: 255,
        };

        // Serialize the data
        let serialized_data: Vec<u8> = original_packet_header.serialize().unwrap();

        // Deserialize the serialized data
        let deserialized_packet_header: PacketHeader = PacketHeader::unserialize(&serialized_data).unwrap();

        // Check if the deserialized data matches the original data
        assert_eq!(original_packet_header, deserialized_packet_header);
    }
}
