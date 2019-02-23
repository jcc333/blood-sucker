use std::io::{Error, ErrorKind, Read, Result, Write};
use mqtt::*;

pub type PacketId = u16;

pub enum VariableHeader {
    Connect {
        username: bool,
        password: bool,
        will_retain: bool,
        will_qos: QualityOfService,
        will_flag: bool,
        clean_session: bool,
        keep_alive: u16
    },
    Connack {
        session_present: bool,
        return_code: ConnackReturnCode
    },
    Publish {
        topic_name: String,
        packet_id: Option<PacketId>
    },
    Puback(PacketId),
    Pubrec(PacketId),
    Pubrel(PacketId),
    Pubcomp(PacketId),
    Subscribe(PacketId),
    Suback(PacketId),
    Unsubscribe(PacketId),
    Unsuback(PacketId),
}

impl VariableHeader {
    pub fn len(&self) -> u32 {
        match self {
            VariableHeader::Connect{
                username:_,
                password: _,
                will_retain: _,
                will_qos: _,
                will_flag: _,
                clean_session: _,
                keep_alive: _
            } => 10u32,
            VariableHeader::Publish{ topic_name, packet_id: Some(_) } =>
                (topic_name.len() + 2) as u32,
            VariableHeader::Publish{ topic_name, packet_id: None } =>
                topic_name.len() as u32,
            _ => 2u32
        }
    }
}

impl Serde for VariableHeader {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "not implemented"))
    }

    fn de(source: &mut Read) -> Result<(Self, usize)> {
        Err(Error::new(ErrorKind::Other, "not implemented"))
    }
}
