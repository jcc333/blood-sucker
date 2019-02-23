use mqtt::*;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::convert::TryFrom;

pub struct Will {
    retain: bool,
    qos: QualityOfService,
    topic: String,
    message: String
}

pub enum Message {
    Connect {
        client_id: String,
        username: String,
        password: String,
        will: Option<Will>,
        clean_session: bool,
        keep_alive: u16,
    },
    Connack {
        session_present: bool,
        return_code: ConnackReturnCode
    },
    Publish {
        dup: bool,
        qos: QualityOfService,
        retain: bool,
        topic: String,
        packet_id: Option<PacketId>,
        payload: String
    },
    Puback(PacketId),
    Pubrec(PacketId),
    Pubrel(PacketId),
    Pubcomp(PacketId),
    Subscribe {
        packet_id: PacketId,
        topic_filters: Vec<(String, QualityOfService)>
    },
    Suback {
        packet_id: PacketId,
        return_codes: Vec<Option<QualityOfService>>
    },
    Unsubscribe {
        packet_id: PacketId,
        topic_filters: Vec<String>
    },
    Unsuback(PacketId),
    Pingreq,
    Pingresp,
    Disconnect
}

impl Message {
    fn packet_type(&self) -> ControlPacketType {
        match *self {
            Message::Connect { client_id: _, username: _, password: _,
                               will: _, clean_session: _, keep_alive: _ } =>
                ControlPacketType::Connect,
            Message::Connack { session_present: _, return_code: _ } =>
                ControlPacketType::Connack,
            Message::Publish { dup: _, qos: _, retain: _, topic: _, packet_id: _, payload: _ } =>
                ControlPacketType::Publish,
            Message::Puback(_) =>
                ControlPacketType::Puback,
            Message::Pubrec(_) =>
                ControlPacketType::Pubrec,
            Message::Pubrel(_) =>
                ControlPacketType::Pubrel,
            Message::Pubcomp(_) =>
                ControlPacketType::Pubcomp,
            Message::Subscribe { packet_id: _, topic_filters: _ } =>
                ControlPacketType::Subscribe,
            Message::Suback { packet_id: _, return_codes: _ } =>
                ControlPacketType::Suback,
            Message::Unsubscribe { packet_id: _, topic_filters: _ } =>
                ControlPacketType::Unsubscribe,
            Message::Unsuback(_) => ControlPacketType::Unsuback,
            Message::Pingreq => ControlPacketType::Pingreq,
            Message::Pingresp => ControlPacketType::Pingresp,
            Message::Disconnect => ControlPacketType::Disconnect
        }
    }

    fn flags(&self) -> [bool; 4] {
        match self {
            Message::Publish { dup, qos, retain, topic, packet_id, payload } => {
                let (qos0, qos1) = qos.bits();
                [*dup, qos0, qos1, *retain]
            },
            Message::Pubrel(_) => [false, false, true, false],
            Message::Subscribe { packet_id, topic_filters } => [false, false, true, false],
            Message::Unsubscribe { packet_id, topic_filters } => [false, false, true, false],
            _ => [false, false, false, false]
        }
    }

    fn remaining_length(
        vho: &Option<VariableHeader>,
        plo: &Option<Payload>
    ) -> Result<RemainingLength> {
        let vh_len = vho.as_ref().map_or(0, |v| { v.len() }) as u32;
        let pl_len = match plo {
            None => 0u32,
            Some(plo) => plo.len() as u32
        };
         //   plo.map_or(0, |p| { p.len() as u32 }) as u32;
        RemainingLength::try_from((vh_len + pl_len) as u32)
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        match self {
            Message::Connect {
                client_id,
                username,
                password,
                will,
                clean_session,
                keep_alive
            } => {
                let (retain, qos, flag) = match will {
                    Some(Will{ retain, qos, topic: _, message: _ }) =>
                        (*retain, *qos, true),
                    None =>
                        (false, QualityOfService::AtMostOnce, false)
                };
                Some(VariableHeader::Connect {
                    username: username.len() != 0,
                    password: password.len() != 0,
                    will_retain: retain,
                    will_qos: qos,
                    will_flag: flag,
                    clean_session: *clean_session,
                    keep_alive: *keep_alive
                })
            },
            Message::Connack { session_present, return_code } =>
                Some(VariableHeader::Connack {
                    session_present: *session_present,
                    return_code: *return_code
                }),
            Message::Publish {
                dup,
                qos,
                retain,
                topic,
                packet_id,
                payload
            } =>
                Some(VariableHeader::Publish{
                    topic_name: topic.to_string(),
                    packet_id: *packet_id
                }),
            Message::Subscribe{ packet_id, topic_filters } =>
                Some(VariableHeader::Subscribe(*packet_id)),
            Message::Suback{ packet_id, return_codes } =>
                Some(VariableHeader::Suback(*packet_id)),
            Message::Unsubscribe { packet_id, topic_filters } =>
                Some(VariableHeader::Unsubscribe(*packet_id)),
            Message::Unsuback(packet_id) =>
                Some(VariableHeader::Unsuback(*packet_id)),
            Message::Puback(packet_id) => Some(VariableHeader::Puback(*packet_id)),
            Message::Pubrec(packet_id) => Some(VariableHeader::Pubrec(*packet_id)),
            Message::Pubrel(packet_id) => Some(VariableHeader::Pubrel(*packet_id)),
            Message::Pubcomp(packet_id) => Some(VariableHeader::Pubcomp(*packet_id)),
            _ => None
        }
    }

    fn payload(&self) -> Option<Payload> {
        match self {
            Message::Connect{
                client_id,
                username,
                password,
                will,
                clean_session: _,
                keep_alive: _
            } => {
                let will_pair = match will {
                    Some(Will{ retain: _, qos: _, topic, message }) =>
                        Some((topic.clone(), message.clone())),
                    None => None
                };
                Some(Payload::Connect{
                    client_id,
                    will: will_pair,
                    username,
                    password
                })
            },
            Message::Publish { dup: _, qos: _, retain: _, topic: _, packet_id: _, payload } =>
                Some(Payload::Publish(payload)),
            Message::Subscribe { packet_id: _, topic_filters } => 
                Some(Payload::Subscribe(topic_filters)),
            Message::Suback { packet_id: _, return_codes } =>
                Some(Payload::Suback(return_codes)),
            Message::Unsubscribe { packet_id: _, topic_filters } =>
                Some(Payload::Unsubscribe(topic_filters)),
            _ => None
        }
    }
}

impl Serde for Message {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        let control_packet_type = self.packet_type();
        let flags = self.flags();
        let variable_header = self.variable_header(); // vho
        let payload = self.payload(); // plo
        let remaining_length = Message::remaining_length(
            &variable_header,
            &payload
        )?;
        let fixed_header = FixedHeader{ control_packet_type, flags, remaining_length };
        let mut written = fixed_header.ser(sink)?;
        written += variable_header.map_or(Ok(0), |v| { v.ser(sink) })?;
        written += payload.map_or(Ok(0), |p| { p.ser(sink) })?;
        Ok(written)
    }

    fn de(source: &mut Read) -> Result<(Self, usize)> {
        let (fixed_header, _) = FixedHeader::de(source)?;
        match fixed_header.control_packet_type {
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
}

fn raise_reserved<T>(msg: &str) -> Result<T> {
    Err(Error::new(ErrorKind::InvalidData, msg))
}

fn unimplemented_error<T>() -> Result<T> {
    Err(Error::new(ErrorKind::Other, "not implemented yet"))
}
