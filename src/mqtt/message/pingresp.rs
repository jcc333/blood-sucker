use mqtt::*;

pub struct Pingresp{}

impl Message for Pingresp {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Pingresp
    }
}
