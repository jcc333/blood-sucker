use mqtt::*;

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
