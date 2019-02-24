use mqtt::*;

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
