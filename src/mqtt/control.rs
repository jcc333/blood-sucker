use std::io::{Error, ErrorKind};
use std::io::Read;
use std::io::Result;
use std::io::Write;
use mqtt::*;

pub struct Packet<'a> {
    pub fixed_header: FixedHeader,
    pub variable_header: Option<VariableHeader<'a>>,
    pub payload: Option<Vec<u8>>
}

// https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Figure_2.2_-
// https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Table_2.4_Size
pub struct FixedHeader {
    pub control_packet_type: ControlPacketType,
    pub flags: [bool; 4],
    pub remaining_length: u32, // actually a lower-bounded type encoded in up to 4 bytes
}

impl Serde for FixedHeader {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        let remaining_length_bytes =
            RemainingLength::encode(self.remaining_length)
            .map(|r| { r.bytes() })?;
        let rem_len_written = sink.write(&remaining_length_bytes)?;

        let ctrl_bits = self.control_packet_type.to_byte();
        let ctrl_bits_written = sink.write(&[ctrl_bits])?;

        let flag_bits = self.flags
            .iter()
            .enumerate()
            .fold(0_u8, |acc, (idx, bit)| { acc + (*bit as u8) << (4_u8 - idx as u8) });
        let flag_bits_written = sink.write(&[flag_bits])?;

        Ok(rem_len_written + ctrl_bits_written + flag_bits_written)
    }

    fn de(_source: &mut Read) -> Result<(Self, usize)> {
        Err(Error::new(ErrorKind::Other, "not implemented"))
    }
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
    fn to_byte(&self) -> u8 {
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
