use std::io::{Error, ErrorKind, Result};
use mqtt::*;

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

    fn from_bytes(_bytes: &[u8]) -> Result<u8> {
        Err(Error::new(ErrorKind::Other, "Not yet implemented".to_string()))
    }
}
