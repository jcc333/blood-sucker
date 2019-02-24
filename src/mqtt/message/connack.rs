use mqtt::*;

pub struct Connack {
    session_present: bool,
    return_code: ConnackReturnCode
}

impl Message for Connack {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Connack
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        Some(VariableHeader::Connack {
            session_present: self.session_present,
            return_code: self.return_code
        })
    }
}
