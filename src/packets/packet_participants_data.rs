use super::packet_header::PacketHeader;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParticipantData {
    pub ai_controlled: u8,     // 1 Byte
    pub driver_id: u8,         // 1 Byte
    pub network_id: u8,        // 1 Byte
    pub team_id: u8,           // 1 Byte
    pub my_team: u8,           // 1 Byte
    pub race_number: u8,       // 1 Byte
    pub nationality: u8,       // 1 Byte
    pub name: [u8; 48],        // 48 Bytes
    pub your_telemetry: u8,    // 1 Byte
    pub show_online_names: u8, // 1 Byte
    pub platform: u8,          // 1 Byte
} // 58 Bytes

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PacketParticipantsData {
    pub header: PacketHeader,                // 29 Bytes
    pub num_active_cars: u8,                 // 1 Byte
    pub participants: [ParticipantData; 22], // 1276 Bytes
} // 1306 Bytes

impl Default for ParticipantData {
    fn default() -> Self {
        ParticipantData {
            ai_controlled: 0u8,
            driver_id: 0u8,
            network_id: 0u8,
            team_id: 0u8,
            my_team: 0u8,
            race_number: 0u8,
            nationality: 0u8,
            name: [0u8; 48],
            your_telemetry: 0u8,
            show_online_names: 0u8,
            platform: 0u8,
        }
    }
}

impl Default for PacketParticipantsData {
    fn default() -> Self {
        PacketParticipantsData {
            header: PacketHeader::default(),
            num_active_cars: 0u8,
            participants: [ParticipantData::default(); 22],
        }
    }
}

impl ParticipantData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(ParticipantData {
            ai_controlled: cursor.read_u8()?,
            driver_id: cursor.read_u8()?,
            network_id: cursor.read_u8()?,
            team_id: cursor.read_u8()?,
            my_team: cursor.read_u8()?,
            race_number: cursor.read_u8()?,
            nationality: cursor.read_u8()?,
            name: {
                let mut name: [u8; 48] = [0u8; 48];
                for i in 0..48 {
                    name[i] = cursor.read_u8()?;
                }
                name
            },
            your_telemetry: cursor.read_u8()?,
            show_online_names: cursor.read_u8()?,
            platform: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<ParticipantData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.ai_controlled)?;
        cursor.write_u8(self.driver_id)?;
        cursor.write_u8(self.network_id)?;
        cursor.write_u8(self.team_id)?;
        cursor.write_u8(self.my_team)?;
        cursor.write_u8(self.race_number)?;
        cursor.write_u8(self.nationality)?;
        for element in self.name {
            cursor.write_u8(element)?;
        }
        cursor.write_u8(self.your_telemetry)?;
        cursor.write_u8(self.show_online_names)?;
        cursor.write_u8(self.platform)?;

        Ok(bytes)
    }
}

impl PacketParticipantsData {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&bytes[size_of::<PacketHeader>()..]);

        Ok(PacketParticipantsData {
            header: PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?,
            num_active_cars: cursor.read_u8()?,
            participants: {
                let mut participants: [ParticipantData; 22] = [ParticipantData::default(); 22];
                for i in 0..22 {
                    participants[i] = ParticipantData::unserialize(
                        &bytes[size_of::<PacketHeader>()
                            + 1 * size_of::<u8>()
                            + i * size_of::<ParticipantData>()
                            ..size_of::<PacketHeader>()
                                + 1 * size_of::<u8>()
                                + (i + 1) * size_of::<ParticipantData>()],
                    )?;
                }
                participants
            },
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<PacketParticipantsData>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_all(&self.header.serialize()?)?;
        cursor.write_u8(self.num_active_cars)?;
        for element in self.participants {
            cursor.write_all(&element.serialize()?)?;
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Alphanumeric, Rng};

    #[test]
    fn test_participant_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_data: ParticipantData = ParticipantData {
            ai_controlled: rng.gen(),
            driver_id: rng.gen(),
            network_id: rng.gen(),
            team_id: rng.gen(),
            my_team: rng.gen(),
            race_number: rng.gen(),
            nationality: rng.gen(),
            name: [rng.sample(Alphanumeric); 48],
            your_telemetry: rng.gen(),
            show_online_names: rng.gen(),
            platform: rng.gen(),
        };

        let serialized_data: Vec<u8> = original_data.serialize().unwrap();
        let deserialized_data: ParticipantData =
            ParticipantData::unserialize(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }

    #[test]
    fn test_packet_participants_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_packet: PacketParticipantsData = PacketParticipantsData {
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
            num_active_cars: rng.gen(),
            participants: [ParticipantData {
                ai_controlled: rng.gen(),
                driver_id: rng.gen(),
                network_id: rng.gen(),
                team_id: rng.gen(),
                my_team: rng.gen(),
                race_number: rng.gen(),
                nationality: rng.gen(),
                name: [rng.sample(Alphanumeric); 48],
                your_telemetry: rng.gen(),
                show_online_names: rng.gen(),
                platform: rng.gen(),
            }; 22],
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketParticipantsData =
            PacketParticipantsData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
