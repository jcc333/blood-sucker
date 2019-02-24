extern crate byteorder;

use mqtt::*;
use std::io::{Error, ErrorKind, Read, Result, Write};
use self::byteorder::{BigEndian, WriteBytesExt};

pub type PacketId = u16;

pub trait VariableHeader<'a> : Serde {
    pub fn len(&self) -> u16;
}

pub enum VariableHeader<'a> {
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
        topic_name: &'a str,
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

impl<'a> VariableHeader<'a> {
    pub fn len(&self) -> u16 {
        match self {
            VariableHeader::Connect{
                username:_,
                password: _,
                will_retain: _,
                will_qos: _,
                will_flag: _,
                clean_session: _,
                keep_alive: _
            } => 10,
            VariableHeader::Publish{ topic_name, packet_id: Some(_) } =>
                (topic_name.len() + 2) as u16,
            VariableHeader::Publish{ topic_name, packet_id: None } =>
                topic_name.len() as u16,
            _ => 2
        }
    }
}

impl<'a> Serde for VariableHeader<'a> {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        let len = self.len();
        sink.write_u16::<BigEndian>(len)?;
        sink.write("mqtt".as_bytes())?;
        sink.write(&[4u8])?;
        let mut flags_byte = 0u8;
        let reserved = 0;
        let clean_session = self.clean_session;
        sink.write()
    }

    fn de(source: &mut Read) -> Result<(Self, usize)> {
        Err(Error::new(ErrorKind::Other, "not implemented"))
    }
}
