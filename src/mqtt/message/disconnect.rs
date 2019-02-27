use mqtt::*;

pub struct Disconnect{}

impl Message for Disconnect {
    fn packet_type(&self) -> ControlPacketType {
        ControlPacketType::Disconnect
    }
}
