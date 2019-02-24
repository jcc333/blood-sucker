use mqtt::*;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::convert::TryFrom;

mod connack;
pub use self::connack::*;

mod connect;
pub use self::connect::*;

mod pingreq;
pub use self::pingreq::*;

mod pingresp;
pub use self::pingresp::*;

mod puback;
pub use self::puback::*;

mod pubcomp;
pub use self::pubcomp::*;

mod publish;
pub use self::publish::*;

mod pubrec;
pub use self::pubrec::*;

mod pubrel;
pub use self::pubrel::*;

mod suback;
pub use self::suback::*;

mod subscribe;
pub use self::subscribe::*;

mod unsuback;
pub use self::unsuback::*;

mod unsubscribe;
pub use self::unsubscribe::*;

/// A Message represents an MQTT control packet as per
/// https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718018
pub trait Message {
    fn packet_type(&self) -> ControlPacketType;

    fn fixed_header(&self) -> Result<FixedHeader> {
        let remaining_length = self.remaining_length(&None, &None)?;
        Ok(FixedHeader{
            packet_type: self.packet_type(),
            flags: self.flags(),
            remaining_length: remaining_length
        })
    }

    fn flags(&self) -> [bool; 4] {
        [false, false, false, false]
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        None
    }

    fn payload(&self) -> Option<Payload> {
        None
    }

    fn remaining_length(
        &self,
        vho: &Option<VariableHeader>,
        plo: &Option<Payload>
    ) -> Result<RemainingLength> {
        let vho = vho.or(self.variable_header());
        let plo = plo.or(self.payload());
        let vh_len = vho.map_or(0, |v| { v.len() }) as u32;
        let pl_len = match plo {
            None => 0u32,
            Some(plo) => plo.len() as u32
        };
        RemainingLength::try_from((vh_len + pl_len) as u32)
    }
}

pub struct Disconnect{}
impl Message for Disconnect {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Disconnect
    }
}

pub enum EnumeratedMessage {
    EConnect(message::Connect),
    EConnack(message::Connack),
    EPublish(message::Publish),
    EPuback(message::Puback),
    EPubrec(message::Pubrec),
    EPubrel(message::Pubrel),
    EPubcomp(message::Pubcomp),
    ESubscribe(message::Subscribe),
    ESuback(message::Suback),
    EUnsubscribe(message::Unsubscribe),
    EUnsuback(message::Unsuback),
    EPingreq(message::Pingreq),
    EPingresp(message::Pingresp),
    EDisconnect(message::Disconnect)
}

impl EnumeratedMessage {
    fn message(&self) -> &dyn Message {
        match self {
            EnumeratedMessage::EConnect(m) => m,
            EnumeratedMessage::EConnack(m) => m,
            EnumeratedMessage::EPublish(m) => m,
            EnumeratedMessage::EPuback(m) => m,
            EnumeratedMessage::EPubrec(m) => m,
            EnumeratedMessage::EPubrel(m) => m,
            EnumeratedMessage::EPubcomp(m) => m,
            EnumeratedMessage::ESubscribe(m) => m,
            EnumeratedMessage::ESuback(m) => m,
            EnumeratedMessage::EUnsubscribe(m) => m,
            EnumeratedMessage::EUnsuback(m) => m,
            EnumeratedMessage::EPingreq(m) => m,
            EnumeratedMessage::EPingresp(m) => m,
            EnumeratedMessage::EDisconnect(m) => m
        }
    }
}

impl Serde for EnumeratedMessage {
    fn de(source: &mut Read) -> Result<(EnumeratedMessage, usize)> {
        let (fixed_header, _) = FixedHeader::de(source)?;
        match fixed_header.packet_type {
            ControlPacketType::ReservedLow =>
                raise_reserved("Cannot use control-packet-type 0, 'reserved low'"),
            ControlPacketType::Connect =>
                unimplemented_error(),
            ControlPacketType::Connack =>
                unimplemented_error(),
            ControlPacketType::Publish =>
                unimplemented_error(),
            ControlPacketType::Puback =>
                unimplemented_error(),
            ControlPacketType::Pubrec =>
                unimplemented_error(),
            ControlPacketType::Pubrel =>
                unimplemented_error(),
            ControlPacketType::Pubcomp =>
                unimplemented_error(),
            ControlPacketType::Subscribe =>
                unimplemented_error(),
            ControlPacketType::Suback =>
                unimplemented_error(),
            ControlPacketType::Unsubscribe =>
                unimplemented_error(),
            ControlPacketType::Unsuback =>
                unimplemented_error(),
            ControlPacketType::Pingreq =>
                unimplemented_error(),
            ControlPacketType::Pingresp =>
                unimplemented_error(),
            ControlPacketType::Disconnect =>
                unimplemented_error(),
            ControlPacketType::ReservedHigh =>
                raise_reserved("Cannot use control-packet-type 15, 'reserved high'")
        }
    }

    fn ser(&self, sink: &mut Write) -> Result<usize> {
        let msg = self.message();
        let packet_type = msg.packet_type();
        let flags = msg.flags();
        let variable_header = msg.variable_header();
        let payload = msg.payload();
        let remaining_length = msg.remaining_length(
            &variable_header,
            &payload
        )?;
        let fixed_header = FixedHeader{ packet_type, flags, remaining_length };
        let mut written = fixed_header.ser(sink)?;
        written += variable_header.map_or(Ok(0), |v| { v.ser(sink) })?;
        written += payload.map_or(Ok(0), |p| { p.ser(sink) })?;
        Ok(written)
    }            
}

fn raise_reserved<T>(msg: &str) -> Result<T> {
    Err(Error::new(ErrorKind::InvalidData, msg))
}

fn unimplemented_error<T>() -> Result<T> {
    Err(Error::new(ErrorKind::Other, "not implemented yet"))
}
