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
