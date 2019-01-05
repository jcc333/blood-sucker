use std::io::{Error, ErrorKind, Read, Result, Write};
use mqtt::*;

pub type PacketId = u16;

#[derive(Copy, Clone)]
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

impl<'a> Serde for VariableHeader<'a> {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "not implemented"))
    }

    fn de(source: &mut Read) -> Result<(Self, usize)> {
        Err(Error::new(ErrorKind::Other, "not implemented"))
    }
}
