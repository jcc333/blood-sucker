use mqtt::*;

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
