use mqtt::*;

pub struct Pingreq{}

impl Message for Pingreq {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Pingreq
    }
}
