use std::io;
use std::io::prelude::*;

struct Packet {
    fixed_header: FixedHeader,
    variable_header: Option<VariableHeader>,
    payload: Option<Payload>
}

// https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Table_2.4_Size
struct FixedHeader {
    control_packet_type: ControlPacketType,
    flags: [bool; 4],
    remaining_length: RemainingLength
}

impl serde::Serde for FixedHeader {
    fn ser(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        let ctrl_bits = self.control_packet_type.to_byte();
        let flag_bits = self.flags
            .iter()
            .enumerate()
            .fold(0_u8, |acc, (idx, bit)| { acc + (bit as u8) << (4_u8 - idx) });
        match self.remaining_length  {
            One(zero) => bytes.push(zero),
            Two(zero, one) => {
                bytes.push(zero);
                bytes.push(one);
            },
            One(zero, one, two) => {
                bytes.push(zero);
                bytes.push(one);
                bytes.push(two);
            },
            One(zero, one, two, three) => {
                bytes.push(zero);
                bytes.push(one);
                bytes.push(two);
                bytes.push(three);
            }
        }
        bytes
    }

    fn de(bytes: &[u8]) -> std::io::Result<Packet, &str> {
        Err("not implemented")
    }
}

enum ControlPacketType {
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

enum RemainingLength {
    One(u8),
    Two(u8, u8),
    Three(u8, u8, u8),
    Four(u8, u8, u8, u8)
}

enum VariableHeader {
    Connect {
        user_name: bool,
        password: bool,
        will_retain: bool,
        will_qos: bool,
        will_flag: bool,
        clean_session: bool,
        keep_alive: u16
    },
    Connack {
        session_present: bool,
        return_code: ConnackReturnCode
    },
    Publish {
        topic_name: &str,
        packet_id: Option<u16>
    },
    Puback      {  packet_id: Option<u16> },
    Pubrec      {  packet_id: Option<u16> },
    Pubrel      {  packet_id: Option<u16> },
    Pubcomp     {  packet_id: Option<u16> },
    Subscribe   {  packet_id: Option<u16> },
    Suback      {  packet_id: Option<u16> },
    Unsubscribe {  packet_id: Option<u16> },
    Unsuback    {  packet_id: Option<u16> },
}

enum ConnackReturnCode {
    /* 0 */    Accepted,
    /* 1 */    UnacceptableProtocolVersion,
    /* 2 */    IdentifierRejected,
    /* 3 */    ServerUnavailable,
    /* 4 */    BadUsernameOrPassword,
    /* 5 */    NotAuthorized,
}

impl ControlPacketType {
    fn to_byte(self) -> u8 {
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

    fn from_bytes(&[u8]) -> Result<u8, String> {
        Err("Not yet implemented".to_string())
    }
}

impl FixedHeader {
    fn serialize(self) -> [u8; 2] {
        control_packet_type
    }

    fn flags(self, dup: bool, qos: qos::QualityOfService, retain: bool) -> [bool; 4] {
        return match self {
            ControlPacketType::Publish => {
                let mut flags = [false, false, false, false];
                flags[0] = dup;
                match qos {
                    qos::QualityOfService::AtMostOnce => {
                        flags[1] = false;
                        flags[2] = false;
                    },
                    qos::QualityOfService::AtLeastOnce => {
                        flags[1] = false;
                        flags[2] = true;
                    },
                    qos::QualityOfService::ExactlyOnce => {
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

impl RemainingLength {
    fn bytes() -> Vec<u8> {
        match self {
            One(b0) => vec!(b0),
            Two(b0, b1) => vec!(b0, b1),
            Three(b0, b1, b2) => vec!(b0, b1, b2),
            Four(b0, b1, b2, b3) => vec!(b0, b1, b2, b3),
        }
    }
}

impl TryFrom<u32> for RemainingLength {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const MAX_SIZE: u32 = 268_435_455;
        match value {
            0 => Ok(vec![0x00]),
            _ if value > MAX_SIZE => Err("Out of range".to_string()),
            _  => {
                let mut output = Vec::<u8>::new();
                let mut x = value;
                while x > 0 {
                    let mut encoded: u8 = (x % 128) as u8;
                    x = x / 128;
                    if x > 0 {
                        encoded = encoded | 128u8;
                    }
                    output.push(encoded);
                }
                Ok(output)
            }
        }
    }
}

impl Into<u32> for RemainingLength {
    fn into(self) -> u32 {
        self.bytes()
            .iter()
            .enumerate()
            .fold(0, |value, (idx, byte)| {
                let multiplier = 128_u32.pow((idx + 1) as u32);
                let byte_value = (byte & 127_u8) as u32;
                (value as u32) + byte_value * multiplier
            })

    }
}
