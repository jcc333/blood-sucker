use std::io::{Error, ErrorKind, Read, Result, Write};
use mqtt::*;

pub struct Packet<'a> {
    pub fixed_header: FixedHeader,
    pub variable_header: Option<VariableHeader<'a>>,
    pub payload: Option<Vec<u8>>
}

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

impl FixedHeader {
    fn flags(&self, dup: bool, qos: QualityOfService, retain: bool) -> [bool; 4] {
        match self.control_packet_type {
            ControlPacketType::Publish => {
                let mut flags = [false, false, false, false];
                flags[0] = dup;
                match qos {
                    QualityOfService::AtMostOnce => {
                        flags[1] = false;
                        flags[2] = false;
                    },
                    QualityOfService::AtLeastOnce => {
                        flags[1] = false;
                        flags[2] = true;
                    },
                    QualityOfService::ExactlyOnce => {
                        flags[1] = true;
                        flags[2] = false;
                    }
                }
                flags[3] = retain;
                flags
            },
            ControlPacketType::Pubrel => [false, false, true, false],
            ControlPacketType::Subscribe => [false, false, true, false],
            ControlPacketType::Unsubscribe => [false, false, true, false],
            _ => [false, false, false, false]
        }
    }
}
