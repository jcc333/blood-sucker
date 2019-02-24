use mqtt::*;

pub struct Suback {
    packet_id: PacketId,
    return_codes: Vec<Option<QualityOfService>>
}

impl Message for Suback {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Suback
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Suback(self.packet_id))
    }

    fn payload(&self) -> Option<Payload> {
        Some(Payload::Suback(&self.return_codes))
    }
}
