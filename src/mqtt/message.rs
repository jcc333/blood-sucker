use mqtt::*;
use std::io::{Error, ErrorKind, Read, Result, Write};

pub struct Will<'a> {
    retain: bool,
    qos: QualityOfService,
    topic: &'a str,
    message: &'a str
}

impl Will {
    pub fn qos<'a>(will: Option<Will<'a>>) -> QualityOfService {
        will.map(|w| { w.qos }).or_else(QualityOfService::AtMostOnce)
    }

    pub fn retain<'a>(will: Option<Will<'a>>) -> bool {
        will.map(|w| { w.retain }).or_else(false)
    }

    pub fn message_by_topic<'a>(will: Option<Will<'a>>) {
        will.map(|w| { (w.topic, w.message) })
    }
}

pub enum Message<'a> {
    Connect {
        client_id: &'a str,
        username: &'a str,
        password: &'a str,
        will: Option<Will<&'a>>,
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
        topic: &'a str,
        packet_id: Option<PacketId>,
        payload: &'a str
    },
    Puback { packet_id: PacketId },
    Pubrec { packet_id: PacketId },
    Pubrel { packet_id: PacketId },
    Pubcomp { packet_id: PacketId },
    Subscribe {
        packet_id: PacketId,
        topic_filters: Vec<(&'a str, QualityOfService)>
    },
    Suback {
        packet_id: PacketId,
        return_codes: Vec<Option<QualityOfService>>
    },
    Unsubscribe {
        packet_id: PacketId,
        topic_filters: Vec<&'a str>
    },
    Unsuback { packet_id: PacketId },
    Pingreq,
    Pingresp,
    Disconnect
}

impl Message {
    fn packet_type(&self) -> ControlPacketType {
        match *self {
            Message::Connect { _c, _u, _p, _w, _cl, _k } => ControlPacketType::Connect,
            Message::Connack { _s, _r } => ControlPacketType::Connack,
            Message::Publish { _d, _q, _r, _t, _p, _pl } => ControlPacketType::Publish,
            Message::Puback { _p } => ControlPacketType::Puback,
            Message::Pubrec { _p } => ControlPacketType::Pubrec,
            Message::Pubrel { _p } => ControlPacketType::Pubrel,
            Message::Pubcomp { _p} => ControlPacketType::Pubcomp,
            Message::Subscribe { _p, _t } => ControlPacketType::Subscribe,
            Message::Suback { _p, _r } => ControlPacketType::Suback,
            Message::Unsubscribe { _p, _t } => ControlPacketType::Unsubscribe,
            Message::Unsuback { _p } => ControlPacketType::Unsuback,
            Message::Pingreq => ControlPacketType::Pingreq,
            Message::Pingresp => ControlPacketType::Pingresp,
            Message::Disconnect => ControlPacketType::Disconnect
        }
    }

    fn flags(&self) -> [bool; 4] {
        match *self {
            Message::Publish { dup, qos, retain, _topic, _packet, _payload } => {
                let (qos0, qos1) = qos.bits();
                [dup, qos0, qos1, retain]
            },
            Message::Pubrel { _p } => [false, false, true, false],
            Message::Subscribe { _p, _f } => [false, false, true, false],
            Message::Unsubscribe { _p, _f } => [false, false, true, false],
            _ => [false, false, false, false]
        }
    }

    fn fixed_header(&self) -> FixedHeader {
        FixedHeader{
            control_packet_type: self.packet_type(),
            flags: self.flags(),
            remaining_length: self.remaining_length()
        }
    }

    fn variable_header(&self) -> Option<VariableHeader<'a>> {
        match *self { 
            Message::Connect { _c, username, password, will, _cl, _k } =>
                Some(VariableHeader::Connect {
                    username: username.len() != 0,
                    password: password.len() != 0,
                    will_retain: Will::retain(will),
                    will_qos: Will::qos(will),
                    will_flag: will.is_some()
                }),
            Message::Connack { session_present, return_code } =>
                Some(VariableHeader::Connack {
                    session_present: session_present,
                    return_code: return_code,
                }),
            Message::Publish { _d, _q, _r, topic_name, packet_id, _p } =>
                Some(VariableHeader::Publish {
                    topic_name: topic_name,
                    packet_id: packet_id
                }),
            Message::Puback { packet_id } =>
                Some(VariableHeader::Puback { packet_id: packet_id }),
            Message::Pubrec { packet_id } =>
                Some(VariableHeader::Pubrec { packet_id: packet_id }),
            Message::Pubrel { packet_id } =>
                Some(VariableHeader::Pubrel { packet_id: packet_id }),
            Message::Pubcomp { packet_id } =>
                Some(VariableHeader::Pubcomp { packet_id: packet_id }),
            Message::Subscribe { packet_id, topics } =>
                Some(VariableHeader::Subscribe { packet_id: packet_id }),
            Message::Suback { packet_id, return_codes } =>
                Some(VariableHeader::Suback { packet_id: packet }),
            Message::Unsubscribe { packet, topics } =>
                Some(VariableHeader::Unsubscribe { packet_id: packet }),
            Message::Unsuback { packet } =>
                Some(VariableHeader::Unsuback { packet_id: packet }),
            _ => None
        }
    }

    fn payload(&self) -> Option<Payload> {
        match *self {
            Message::Connect { client_id, username, password, will, clean_session, keepalive } =>
                Some(Payload::Connect{
                    client_id,
                    username,
                    password,
                    Will::retain(will),
                    Will::qos(will),
                    Will::message_by_topic(will)
                }),
            Message::Publish { _d, _q, _r, _t, _p, payload } =>
                 Some(Payload::Publish(payload)),
            Message::Subscribe { _p, _t } =>
                Some(),
            Message::Suback { _p, _r } =>
                Some(),
            Message::Unsubscribe { _p, _t } =>
                Some(),
            _ => None
        }
    }
}

impl Serde for Message<'a> {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        let fixed_header_len = self.fixed_header().ser(sink)?;
        let var_header_len = self.variable_header().ser(sink)?;
        let payload_len = self.payload().map(|bytes| sink.write(bytes)).or_else(Ok(0))?;
        fixed_header_len + var_header_len + payload_len;
    }

    fn de(source: &mut Read) -> Result<(Self, usize)> {
        let (fixed_header, fixed_header_len) = FixedHeader::de(source)?;
        match fixed_header.control_packet_type {
            ControlPacketType::ReservedLow => ,
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "Cannot use control-packet-type 0, 'reserved low'"
                ))
            ControlPacketType::Connect => read_connect(fixed_header, source),
            ControlPacketType::Connack => read_connack(fixed_header, source),
            ControlPacketType::Publish => read_publish(fixed_header, source),
            ControlPacketType::Puback => read_puback(fixed_header, source),
            ControlPacketType::Pubrec => read_pubrec(fixed_header, source),
            ControlPacketType::Pubrel => read_pubrel(fixed_header, source),
            ControlPacketType::Pubcomp => read_pubcomp(fixed_header, source),
            ControlPacketType::Subscribe => read_subscribe(fixed_header, source),
            ControlPacketType::Suback => read_suback(fixed_header, source),
            ControlPacketType::Unsubscribe => read_unsubscribe(fixed_header, source),
            ControlPacketType::Unsuback => read_unsuback(fixed_header, source),
            ControlPacketType::Pingreq => read_pingreq(fixed_header, source),
            ControlPacketType::Pingresp => read_pingresp(fixed_header, source),
            ControlPacketType::Disconnect => read_disconnect(fixed_header, source),
            ControlPacketType::ReservedHigh =>
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "Cannot use control-packet-type 15, 'reserved high'"
                ))
        }
    }
}

fn read_connect(fixed_header, source) -> Result<Message> {
}

fn read_connack(fixed_header, source) -> Result<Message> {
}

fn read_publish(fixed_header, source) -> Result<Message> {
}

fn read_puback(fixed_header, source) -> Result<Message> {
}

fn read_pubrec(fixed_header, source) -> Result<Message> {
}

fn read_pubrel(fixed_header, source) -> Result<Message> {
}

fn read_pubcomp(fixed_header, source) -> Result<Message> {
}

fn read_subscribe(fixed_header, source) -> Result<Message> {
}

fn read_suback(fixed_header, source) -> Result<Message> {
}

fn read_unsubscribe(fixed_header, source) -> Result<Message> {
}

fn read_unsuback(fixed_header, source) -> Result<Message> {
}

fn read_pingreq(fixed_header, source) -> Result<Message> {
}

fn read_pingresp(fixed_header, source) -> Result<Message> {
}

fn read_disconnect(fixed_header, source) -> Result<Message> {
}
