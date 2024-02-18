use super::packet_header::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
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
#[derive(Debug, Default, Clone, Copy)]
pub struct FastestLap {
    pub vehicle_idx: u8, // 1 Byte
    pub lap_time: f32,   // 4 Bytes
} // 5 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Retirement {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TeamMateInPits {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RaceWinner {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
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
#[derive(Debug, Default, Clone, Copy)]
pub struct SpeedTrap {
    pub vehicle_idx: u8,                    // 1 Byte
    pub speed: f32,                         // 4 Bytes
    pub is_overall_fastest_in_session: u8,  // 1 Byte
    pub is_driver_fastest_in_session: u8,   // 1 Byte
    pub fastest_vehicle_idx_in_session: u8, // 1 Byte
    pub fastest_speed_in_session: f32,      // 4 Bytes
} // 12 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct StartLights {
    pub num_lights: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct DriveThroughPenaltyServed {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct StopGoPenaltyServed {
    pub vehicle_idx: u8, // 1 Byte
} // 1 Byte

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Flashback {
    pub flashback_frame_identifier: u32, // 4 Bytes
    pub flashback_session_time: f32,     // 4 Bytes
} // 8 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Buttons {
    pub button_status: u32, // 4 Bytes
} // 4 Bytes

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
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
            let mut event_string_code: [u8; 4] = [0; 4];
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
                // Unused Event Details
                fastest_lap: FastestLap::default(),
            },
            b"SEND" => EventDataDetails {
                // Unused Event Details
                fastest_lap: FastestLap::default(),
            },
            b"DRSE" => EventDataDetails {
                // Unused Event Details
                fastest_lap: FastestLap::default(),
            },
            b"DRSD" => EventDataDetails {
                // Unused Event Details
                fastest_lap: FastestLap::default(),
            },
            b"CHQF" => EventDataDetails {
                // Unused Event Details
                fastest_lap: FastestLap::default(),
            },
            b"LGOT" => EventDataDetails {
                // Unused Event Details
                fastest_lap: FastestLap::default(),
            },
            b"RDFL" => EventDataDetails {
                // Unused Event Details
                fastest_lap: FastestLap::default(),
            },
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

        match &self.event_string_code {
            b"FTLP" => unsafe {
                bytes.extend_from_slice(&self.event_details.fastest_lap.serialize()?);
            },
            b"RTMT" => unsafe {
                bytes.extend_from_slice(&self.event_details.retirement.serialize()?);
            },
            b"TMPT" => unsafe {
                bytes.extend_from_slice(&self.event_details.team_mate_in_pits.serialize()?);
            },
            b"RCWN" => unsafe {
                bytes.extend_from_slice(&self.event_details.race_winner.serialize()?);
            },
            b"PENA" => unsafe {
                bytes.extend_from_slice(&self.event_details.penalty.serialize()?);
            },
            b"SPTP" => unsafe {
                bytes.extend_from_slice(&self.event_details.speed_trap.serialize()?);
            },
            b"STLG" => unsafe {
                bytes.extend_from_slice(&self.event_details.start_lights.serialize()?);
            },
            b"DTSV" => unsafe {
                bytes.extend_from_slice(
                    &self
                        .event_details
                        .drive_through_penalty_served
                        .serialize()?,
                );
            },
            b"SGSV" => unsafe {
                bytes.extend_from_slice(&self.event_details.stop_go_penalty_served.serialize()?);
            },
            b"FLBK" => unsafe {
                bytes.extend_from_slice(&self.event_details.flashback.serialize()?);
            },
            b"BUTN" => unsafe {
                bytes.extend_from_slice(&self.event_details.buttons.serialize()?);
            },
            b"OVTK" => unsafe {
                bytes.extend_from_slice(&self.event_details.overtake.serialize()?);
            },
            b"SSTA" => unsafe {
                // Unused Event Details
                bytes.extend_from_slice(&self.event_details.fastest_lap.serialize()?)
            },
            b"SEND" => unsafe {
                // Unused Event Details
                bytes.extend_from_slice(&self.event_details.fastest_lap.serialize()?)
            },
            b"DRSE" => unsafe {
                // Unused Event Details
                bytes.extend_from_slice(&self.event_details.fastest_lap.serialize()?)
            },
            b"DRSD" => unsafe {
                // Unused Event Details
                bytes.extend_from_slice(&self.event_details.fastest_lap.serialize()?)
            },
            b"CHQF" => unsafe {
                // Unused Event Details
                bytes.extend_from_slice(&self.event_details.fastest_lap.serialize()?)
            },
            b"LGOT" => unsafe {
                // Unused Event Details
                bytes.extend_from_slice(&self.event_details.fastest_lap.serialize()?)
            },
            b"RDFL" => unsafe {
                // Unused Event Details
                bytes.extend_from_slice(&self.event_details.fastest_lap.serialize()?)
            },
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid event string code",
                ))
            }
        };

        Ok(bytes)
    }
}
