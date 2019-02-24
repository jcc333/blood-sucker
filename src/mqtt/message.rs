use mqtt::*;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::convert::TryFrom;

mod connack;
pub use self::connack::*;

mod connect;
pub use self::connect::*;

mod publish;
pub use self::publish::*;

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

pub struct Puback {
    packet_id: PacketId
}

impl Message for Puback {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Puback
    }

    fn variable_header(&self) -> Option<VariableHeader> {
      Some(VariableHeader::Puback(self.packet_id))
    }
}

pub struct Pubrec {
    packet_id: PacketId
}

impl Message for Pubrec {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Pubrec
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Pubrec(self.packet_id))
    }
}

pub struct Pubrel {
    packet_id: PacketId
}

impl Message for Pubrel {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Pubrel
    }

    fn flags(&self) -> [bool; 4] {
        [false, false, true, false]
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Pubrel(self.packet_id))
    }
}

pub struct Pubcomp {
    packet_id: PacketId
}

impl Message for Pubcomp {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Pubcomp
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Pubcomp(self.packet_id))
    }
}

pub struct Subscribe {
    packet_id: PacketId,
    topic_filters: Vec<(String, QualityOfService)>
}

impl Message for Subscribe {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Subscribe
    }

    fn flags(&self) -> [bool; 4] {
        [false, false, true, false]
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Subscribe(self.packet_id))
    }

    fn payload(&self) -> Option<Payload> {
        Some(Payload::Subscribe(&self.topic_filters))
    }
}

pub struct Suback {
    packet_id: PacketId,
    return_codes: Vec<Option<QualityOfService>>
}

impl Message for Suback {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Suback
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Suback(self.packet_id))
    }

    fn payload(&self) -> Option<Payload> {
        Some(Payload::Suback(&self.return_codes))
    }
}

pub struct Unsubscribe {
    packet_id: PacketId,
    topic_filters: Vec<String>
}

impl Message for Unsubscribe {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Unsubscribe
    }

    fn flags(&self) -> [bool; 4] {
        [false, false, true, false]
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Unsubscribe(self.packet_id))
    }

    fn payload(&self) -> Option<Payload> {
        Some(Payload::Unsubscribe(&self.topic_filters))
    }
}

pub struct Unsuback {
    packet_id: PacketId
}

impl Message for Unsuback {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Unsuback
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Unsuback(self.packet_id))
    }
}

pub struct Pingreq{}
impl Message for Pingreq {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Pingreq
    }
}

pub struct Pingresp{}
impl Message for Pingresp {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Pingresp
    }
}

pub struct Disconnect{}
impl Message for Disconnect {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Disconnect
    }
}

pub enum WrappedMessage {
    WConnect(message::Connect),
    WConnack(message::Connack),
    WPublish(message::Publish),
    WPuback(message::Puback),
    WPubrec(message::Pubrec),
    WPubrel(message::Pubrel),
    WPubcomp(message::Pubcomp),
    WSubscribe(message::Subscribe),
    WSuback(message::Suback),
    WUnsubscribe(message::Unsubscribe),
    WUnsuback(message::Unsuback),
    WPingreq(message::Pingreq),
    WPingresp(message::Pingresp),
    WDisconnect(message::Disconnect)
}

impl WrappedMessage {
    fn message(&self) -> &dyn Message {
        match self {
            WrappedMessage::WConnect(m) => m,
            WrappedMessage::WConnack(m) => m,
            WrappedMessage::WPublish(m) => m,
            WrappedMessage::WPuback(m) => m,
            WrappedMessage::WPubrec(m) => m,
            WrappedMessage::WPubrel(m) => m,
            WrappedMessage::WPubcomp(m) => m,
            WrappedMessage::WSubscribe(m) => m,
            WrappedMessage::WSuback(m) => m,
            WrappedMessage::WUnsubscribe(m) => m,
            WrappedMessage::WUnsuback(m) => m,
            WrappedMessage::WPingreq(m) => m,
            WrappedMessage::WPingresp(m) => m,
            WrappedMessage::WDisconnect(m) => m
        }
    }
}

impl Serde for WrappedMessage {
    fn de(source: &mut Read) -> Result<(WrappedMessage, usize)> {
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
