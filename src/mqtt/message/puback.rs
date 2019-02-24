use mqtt::*;

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
