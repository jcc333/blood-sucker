use mqtt::*;

pub struct Publish {
    dup: bool,
    qos: QualityOfService,
    retain: bool,
    topic: String,
    packet_id: Option<PacketId>,
    payload: String
}

impl Message for Publish {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Publish
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Publish{
            topic_name: self.topic.to_string(),
            packet_id: self.packet_id
        })
    }

    fn payload(&self) -> Option<Payload> {
        Some(Payload::Publish(&self.payload))
    }

    fn flags(&self) -> [bool; 4] {
        let (qos0, qos1) = self.qos.bits();
        [self.dup, qos0, qos1, self.retain]
    }
}
