use mqtt::*;

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
