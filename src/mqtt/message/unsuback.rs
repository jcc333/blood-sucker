use mqtt::*;

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
