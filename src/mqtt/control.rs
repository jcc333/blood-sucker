use std::io::{Error, ErrorKind, Result};
use std::fmt;

#[derive(Copy, Clone)]
pub enum ControlPacketType {
    ReservedLow,
    Connect,
    Connack,
    Publish,
    Puback,
    Pubrec,
    Pubrel,
    Pubcomp,
    Subscribe,
    Suback,
    Unsubscribe,
    Unsuback,
    Pingreq,
    Pingresp,
    Disconnect,
    ReservedHigh
}

impl ControlPacketType {
    pub fn to_byte(&self) -> u8 {
        match self {
            ControlPacketType::ReservedLow => 0,
            ControlPacketType::Connect => 1,
            ControlPacketType::Connack => 2,
            ControlPacketType::Publish => 3,
            ControlPacketType::Puback => 4,
            ControlPacketType::Pubrec => 5,
            ControlPacketType::Pubrel => 6,
            ControlPacketType::Pubcomp => 7,
            ControlPacketType::Subscribe => 8,
            ControlPacketType::Suback => 9,
            ControlPacketType::Unsubscribe => 10,
            ControlPacketType::Unsuback => 11,
            ControlPacketType::Pingreq => 12,
            ControlPacketType::Pingresp => 13,
            ControlPacketType::Disconnect => 14,
            ControlPacketType::ReservedHigh => 15
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        match bytes[0] {
            0 => Ok(ControlPacketType::ReservedLow),
            1 => Ok(ControlPacketType::Connect),
            2 => Ok(ControlPacketType::Connack),
            3 => Ok(ControlPacketType::Publish),
            4 => Ok(ControlPacketType::Puback),
            5 => Ok(ControlPacketType::Pubrec),
            6 => Ok(ControlPacketType::Pubrel),
            7 => Ok(ControlPacketType::Pubcomp),
            8 => Ok(ControlPacketType::Subscribe),
            9 => Ok(ControlPacketType::Suback),
            10 => Ok(ControlPacketType::Unsubscribe),
            11 => Ok(ControlPacketType::Unsuback),
            12 => Ok(ControlPacketType::Pingreq),
            13 => Ok(ControlPacketType::Pingresp),
            14 => Ok(ControlPacketType::Disconnect),
            15 => Ok(ControlPacketType::ReservedHigh),
            n => {
                let msg = format!("{} is not a valid control packet type: [0, 16)", n);
                Err(Error::new(ErrorKind::InvalidData, msg))
            }
        }
    }
}
