use mqtt::*;

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
