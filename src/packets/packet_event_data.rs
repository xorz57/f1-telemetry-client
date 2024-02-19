use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub union EventDataDetails {
    pub fastest_lap: FastestLap,                                 // 5 Bytes
    pub retirement: Retirement,                                  // 1 Byte
    pub team_mate_in_pits: TeamMateInPits,                       // 1 Byte
    pub race_winner: RaceWinner,                                 // 1 Byte
    pub penalty: Penalty,                                        // 7 Bytes
    pub speed_trap: SpeedTrap,                                   // 12 Bytes
    pub start_lights: StartLights,                               // 1 Byte
    pub drive_through_penalty_served: DriveThroughPenaltyServed, // 1 Byte
    pub stop_go_penalty_served: StopGoPenaltyServed,             // 1 Byte
    pub flashback: Flashback,                                    // 8 Bytes
    pub buttons: Buttons,                                        // 4 Bytes
    pub overtake: Overtake,                                      // 2 Bytes
} // 12 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FastestLap {
    pub vehicle_idx: u8, // 1 Byte
    pub lap_time: f32,   // 4 Bytes
} // 5 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Retirement {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TeamMateInPits {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct RaceWinner {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Penalty {
    pub penalty_type: u8,      // 1 Byte
    pub infringement_type: u8, // 1 Byte
    pub vehicle_idx: u8,       // 1 Byte
    pub other_vehicle_idx: u8, // 1 Byte
    pub time: u8,              // 1 Byte
    pub lap_num: u8,           // 1 Byte
    pub places_gained: u8,     // 1 Byte
} // 7 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SpeedTrap {
    pub vehicle_idx: u8,                    // 1 Byte
    pub speed: f32,                         // 4 Bytes
    pub is_overall_fastest_in_session: u8,  // 1 Byte
    pub is_driver_fastest_in_session: u8,   // 1 Byte
    pub fastest_vehicle_idx_in_session: u8, // 1 Byte
    pub fastest_speed_in_session: f32,      // 4 Bytes
} // 12 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct StartLights {
    pub num_lights: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DriveThroughPenaltyServed {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct StopGoPenaltyServed {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Flashback {
    pub flashback_frame_identifier: u32, // 4 Bytes
    pub flashback_session_time: f32,     // 4 Bytes
} // 8 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Buttons {
    pub button_status: u32, // 4 Bytes
} // 4 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Overtake {
    pub overtaking_vehicle_idx: u8,      // 1 Byte
    pub being_overtaken_vehicle_idx: u8, // 1 Byte
} // 2 Bytes

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketEventData {
    pub header: PacketHeader,            // 29 Bytes
    pub event_string_code: [u8; 4],      // 4 Bytes
    pub event_details: EventDataDetails, // 12 Bytes
} // 45 Bytes

impl FastestLap {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(FastestLap {
            vehicle_idx: cursor.read_u8()?,
            lap_time: cursor.read_f32::<LittleEndian>()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<FastestLap>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.vehicle_idx)?;
        cursor.write_f32::<LittleEndian>(self.lap_time)?;

        Ok(bytes)
    }
}

impl Retirement {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(Retirement {
            vehicle_idx: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<Retirement>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.vehicle_idx)?;

        Ok(bytes)
    }
}

impl TeamMateInPits {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(TeamMateInPits {
            vehicle_idx: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<TeamMateInPits>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.vehicle_idx)?;

        Ok(bytes)
    }
}

impl RaceWinner {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(RaceWinner {
            vehicle_idx: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<RaceWinner>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.vehicle_idx)?;

        Ok(bytes)
    }
}

impl Penalty {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(Penalty {
            penalty_type: cursor.read_u8()?,
            infringement_type: cursor.read_u8()?,
            vehicle_idx: cursor.read_u8()?,
            other_vehicle_idx: cursor.read_u8()?,
            time: cursor.read_u8()?,
            lap_num: cursor.read_u8()?,
            places_gained: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<Penalty>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.penalty_type)?;
        cursor.write_u8(self.infringement_type)?;
        cursor.write_u8(self.vehicle_idx)?;
        cursor.write_u8(self.other_vehicle_idx)?;
        cursor.write_u8(self.time)?;
        cursor.write_u8(self.lap_num)?;
        cursor.write_u8(self.places_gained)?;

        Ok(bytes)
    }
}

impl SpeedTrap {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(SpeedTrap {
            vehicle_idx: cursor.read_u8()?,
            speed: cursor.read_f32::<LittleEndian>()?,
            is_overall_fastest_in_session: cursor.read_u8()?,
            is_driver_fastest_in_session: cursor.read_u8()?,
            fastest_vehicle_idx_in_session: cursor.read_u8()?,
            fastest_speed_in_session: cursor.read_f32::<LittleEndian>()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<SpeedTrap>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.vehicle_idx)?;
        cursor.write_f32::<LittleEndian>(self.speed)?;
        cursor.write_u8(self.is_overall_fastest_in_session)?;
        cursor.write_u8(self.is_driver_fastest_in_session)?;
        cursor.write_u8(self.fastest_vehicle_idx_in_session)?;
        cursor.write_f32::<LittleEndian>(self.fastest_speed_in_session)?;

        Ok(bytes)
    }
}

impl StartLights {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(StartLights {
            num_lights: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<StartLights>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.num_lights)?;

        Ok(bytes)
    }
}

impl DriveThroughPenaltyServed {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(DriveThroughPenaltyServed {
            vehicle_idx: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<DriveThroughPenaltyServed>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.vehicle_idx)?;

        Ok(bytes)
    }
}

impl StopGoPenaltyServed {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(StopGoPenaltyServed {
            vehicle_idx: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<StopGoPenaltyServed>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.vehicle_idx)?;

        Ok(bytes)
    }
}

impl Flashback {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(Flashback {
            flashback_frame_identifier: cursor.read_u32::<LittleEndian>()?,
            flashback_session_time: cursor.read_f32::<LittleEndian>()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<Flashback>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u32::<LittleEndian>(self.flashback_frame_identifier)?;
        cursor.write_f32::<LittleEndian>(self.flashback_session_time)?;

        Ok(bytes)
    }
}

impl Buttons {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(Buttons {
            button_status: cursor.read_u32::<LittleEndian>()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<Buttons>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u32::<LittleEndian>(self.button_status)?;

        Ok(bytes)
    }
}

impl Overtake {
    #[allow(dead_code)]
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);

        Ok(Overtake {
            overtaking_vehicle_idx: cursor.read_u8()?,
            being_overtaken_vehicle_idx: cursor.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(size_of::<Overtake>());
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut bytes);

        cursor.write_u8(self.overtaking_vehicle_idx)?;
        cursor.write_u8(self.being_overtaken_vehicle_idx)?;

        Ok(bytes)
    }
}

impl PacketEventData {
    pub fn unserialize(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let header: PacketHeader = PacketHeader::unserialize(&bytes[..size_of::<PacketHeader>()])?;
        let event_string_code: [u8; 4] = {
            let mut event_string_code: [u8; 4] = [0u8; 4];
            event_string_code.copy_from_slice(
                &bytes[size_of::<PacketHeader>()..size_of::<PacketHeader>() + 4 * size_of::<u8>()],
            );
            event_string_code
        };

        let event_details: EventDataDetails = match &event_string_code {
            b"FTLP" => EventDataDetails {
                fastest_lap: FastestLap::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"RTMT" => EventDataDetails {
                retirement: Retirement::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"TMPT" => EventDataDetails {
                team_mate_in_pits: TeamMateInPits::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"RCWN" => EventDataDetails {
                race_winner: RaceWinner::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"PENA" => EventDataDetails {
                penalty: Penalty::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"SPTP" => EventDataDetails {
                speed_trap: SpeedTrap::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"STLG" => EventDataDetails {
                start_lights: StartLights::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"DTSV" => EventDataDetails {
                drive_through_penalty_served: DriveThroughPenaltyServed::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"SGSV" => EventDataDetails {
                stop_go_penalty_served: StopGoPenaltyServed::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"FLBK" => EventDataDetails {
                flashback: Flashback::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"BUTN" => EventDataDetails {
                buttons: Buttons::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"OVTK" => EventDataDetails {
                overtake: Overtake::unserialize(
                    &bytes[size_of::<PacketHeader>() + 4 * size_of::<u8>()..],
                )?,
            },
            b"SSTA" => EventDataDetails {
                buttons: Buttons::default(),
            }, // Unused Event Details
            b"SEND" => EventDataDetails {
                buttons: Buttons::default(),
            }, // Unused Event Details
            b"DRSE" => EventDataDetails {
                buttons: Buttons::default(),
            }, // Unused Event Details
            b"DRSD" => EventDataDetails {
                buttons: Buttons::default(),
            }, // Unused Event Details
            b"CHQF" => EventDataDetails {
                buttons: Buttons::default(),
            }, // Unused Event Details
            b"LGOT" => EventDataDetails {
                buttons: Buttons::default(),
            }, // Unused Event Details
            b"RDFL" => EventDataDetails {
                buttons: Buttons::default(),
            }, // Unused Event Details
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid event string code",
                ))
            }
        };

        Ok(PacketEventData {
            header,
            event_string_code,
            event_details,
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::with_capacity(size_of::<PacketEventData>());
        bytes.extend_from_slice(&self.header.serialize()?);
        bytes.extend_from_slice(&self.event_string_code);

        unsafe {
            let ed: EventDataDetails = self.event_details;
            match &self.event_string_code {
                b"FTLP" => bytes.extend_from_slice(&ed.fastest_lap.serialize()?),
                b"RTMT" => bytes.extend_from_slice(&ed.retirement.serialize()?),
                b"TMPT" => bytes.extend_from_slice(&ed.team_mate_in_pits.serialize()?),
                b"RCWN" => bytes.extend_from_slice(&ed.race_winner.serialize()?),
                b"PENA" => bytes.extend_from_slice(&ed.penalty.serialize()?),
                b"SPTP" => bytes.extend_from_slice(&ed.speed_trap.serialize()?),
                b"STLG" => bytes.extend_from_slice(&ed.start_lights.serialize()?),
                b"DTSV" => bytes.extend_from_slice(
                    &self
                        .event_details
                        .drive_through_penalty_served
                        .serialize()?,
                ),
                b"SGSV" => bytes.extend_from_slice(&ed.stop_go_penalty_served.serialize()?),
                b"FLBK" => bytes.extend_from_slice(&ed.flashback.serialize()?),
                b"BUTN" => bytes.extend_from_slice(&ed.buttons.serialize()?),
                b"OVTK" => bytes.extend_from_slice(&ed.overtake.serialize()?),
                b"SSTA" => bytes.extend_from_slice(&ed.buttons.serialize()?), // Unused Event Details
                b"SEND" => bytes.extend_from_slice(&ed.buttons.serialize()?), // Unused Event Details
                b"DRSE" => bytes.extend_from_slice(&ed.buttons.serialize()?), // Unused Event Details
                b"DRSD" => bytes.extend_from_slice(&ed.buttons.serialize()?), // Unused Event Details
                b"CHQF" => bytes.extend_from_slice(&ed.buttons.serialize()?), // Unused Event Details
                b"LGOT" => bytes.extend_from_slice(&ed.buttons.serialize()?), // Unused Event Details
                b"RDFL" => bytes.extend_from_slice(&ed.buttons.serialize()?), // Unused Event Details
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid event string code",
                    ))
                }
            }
        };

        Ok(bytes)
    }
}

impl fmt::Debug for PacketEventData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PacketEventData")
            .field("header", &self.header)
            .field("event_string_code", &self.event_string_code)
            .field("event_details", unsafe {
                match &self.event_string_code {
                    b"FTLP" => &self.event_details.fastest_lap,
                    b"RTMT" => &self.event_details.retirement,
                    b"TMPT" => &self.event_details.team_mate_in_pits,
                    b"RCWN" => &self.event_details.race_winner,
                    b"PENA" => &self.event_details.penalty,
                    b"SPTP" => &self.event_details.speed_trap,
                    b"STLG" => &self.event_details.start_lights,
                    b"DTSV" => &self.event_details.drive_through_penalty_served,
                    b"SGSV" => &self.event_details.stop_go_penalty_served,
                    b"FLBK" => &self.event_details.flashback,
                    b"BUTN" => &self.event_details.buttons,
                    b"OVTK" => &self.event_details.overtake,
                    b"SSTA" => &self.event_details.buttons, // Unused Event Details
                    b"SEND" => &self.event_details.buttons, // Unused Event Details
                    b"DRSE" => &self.event_details.buttons, // Unused Event Details
                    b"DRSD" => &self.event_details.buttons, // Unused Event Details
                    b"CHQF" => &self.event_details.buttons, // Unused Event Details
                    b"LGOT" => &self.event_details.buttons, // Unused Event Details
                    b"RDFL" => &self.event_details.buttons, // Unused Event Details
                    _ => &self.event_details.buttons,       // Unused Event Details
                }
            })
            .finish()
    }
}

impl PartialEq for PacketEventData {
    fn eq(&self, other: &Self) -> bool {
        let ed1: EventDataDetails = self.event_details;
        let ed2: EventDataDetails = other.event_details;

        self.header == other.header
            && self.event_string_code == other.event_string_code
            && unsafe {
                match &self.event_string_code {
                    b"FTLP" => ed1.fastest_lap == ed2.fastest_lap,
                    b"RTMT" => ed1.retirement == ed2.retirement,
                    b"TMPT" => ed1.team_mate_in_pits == ed2.team_mate_in_pits,
                    b"RCWN" => ed1.race_winner == ed2.race_winner,
                    b"PENA" => ed1.penalty == ed2.penalty,
                    b"SPTP" => ed1.speed_trap == ed2.speed_trap,
                    b"STLG" => ed1.start_lights == ed2.start_lights,
                    b"DTSV" => ed1.drive_through_penalty_served == ed2.drive_through_penalty_served,
                    b"SGSV" => ed1.stop_go_penalty_served == ed2.stop_go_penalty_served,
                    b"FLBK" => ed1.flashback == ed2.flashback,
                    b"BUTN" => ed1.buttons == ed2.buttons,
                    b"OVTK" => ed1.overtake == ed2.overtake,
                    b"SSTA" => true, // Unused Event Details
                    b"SEND" => true, // Unused Event Details
                    b"DRSE" => true, // Unused Event Details
                    b"DRSD" => true, // Unused Event Details
                    b"CHQF" => true, // Unused Event Details
                    b"LGOT" => true, // Unused Event Details
                    b"RDFL" => true, // Unused Event Details
                    _ => true,       // Unused Event Details
                }
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_packet_event_data_serialization_deserialization() {
        let mut rng = rand::thread_rng();

        let original_event_string_code: [u8; 4] = *b"FTLP";

        let original_event_details: EventDataDetails = EventDataDetails {
            fastest_lap: FastestLap {
                vehicle_idx: rng.gen(),
                lap_time: rng.gen(),
            },
        };

        let original_packet: PacketEventData = PacketEventData {
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
            event_string_code: original_event_string_code,
            event_details: original_event_details,
        };

        let serialized_packet: Vec<u8> = original_packet.serialize().unwrap();
        let deserialized_packet: PacketEventData =
            PacketEventData::unserialize(&serialized_packet).unwrap();

        assert_eq!(original_packet, deserialized_packet);
    }
}
