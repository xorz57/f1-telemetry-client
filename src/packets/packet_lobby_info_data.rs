use super::packet_header::PacketHeader;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LobbyInfoData {
    pub ai_controlled: u8, // 1 Byte
    pub team_id: u8,       // 1 Byte
    pub nationality: u8,   // 1 Byte
    pub platform: u8,      // 1 Byte
    pub name: [u8; 48],    // 48 Bytes
    pub car_number: u8,    // 1 Byte
    pub ready_status: u8,  // 1 Byte
} // 54 Bytes

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketLobbyInfoData {
    pub header: PacketHeader,               // 29 Bytes
    pub num_players: u8,                    // 1 Byte
    pub lobby_players: [LobbyInfoData; 22], // 1188 Bytes
} // 1218 Bytes

impl Default for LobbyInfoData {
    fn default() -> Self {
        LobbyInfoData {
            ai_controlled: 0u8,
            team_id: 0u8,
            nationality: 0u8,
            platform: 0u8,
            name: [0u8; 48],
            car_number: 0u8,
            ready_status: 0u8,
        }
    }
}

impl Default for PacketLobbyInfoData {
    fn default() -> Self {
        PacketLobbyInfoData {
            header: PacketHeader::default(),
            num_players: 0u8,
            lobby_players: [LobbyInfoData::default(); 22],
        }
    }
}

impl LobbyInfoData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(LobbyInfoData {
            ai_controlled: cursor.read_u8()?,
            team_id: cursor.read_u8()?,
            nationality: cursor.read_u8()?,
            platform: cursor.read_u8()?,
            name: {
                let mut name: [u8; 48] = [0u8; 48];
                for i in 0..48 {
                    name[i] = cursor.read_u8()?
                }
                name
            },
            car_number: cursor.read_u8()?,
            ready_status: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<LobbyInfoData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.ai_controlled)?;
        cursor.write_u8(self.team_id)?;
        cursor.write_u8(self.nationality)?;
        cursor.write_u8(self.platform)?;
        for name in self.name {
            cursor.write_u8(name)?;
        }
        cursor.write_u8(self.car_number)?;
        cursor.write_u8(self.ready_status)?;

        Ok(bytes)
    }
}

impl PacketLobbyInfoData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&bytes[..size_of::<PacketHeader>()]);

        Ok(PacketLobbyInfoData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            num_players: cursor.read_u8()?,
            lobby_players: {
                let mut lobby_players: [LobbyInfoData; 22] = [LobbyInfoData::default(); 22];
                for i in 0..22 {
                    lobby_players[i] = LobbyInfoData::unserialize(
                        &bytes[1 * size_of::<u8>() + i * size_of::<LobbyInfoData>()
                            ..1 * size_of::<u8>() + (i + 1) * size_of::<LobbyInfoData>()],
                    )?;
                }
                lobby_players
            },
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketLobbyInfoData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        cursor.write_u8(self.num_players)?;
        for lobby_players in self.lobby_players {
            cursor.write_all(&lobby_players.serialize()?)?;
        }

        Ok(bytes)
    }
}
