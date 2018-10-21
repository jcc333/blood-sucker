use mqtt::*;

pub type PacketId = u16;

pub enum VariableHeader<'a> {
    Connect {
        username: bool,
        password: bool,
        will_retain: bool,
        will_qos: QualityOfService,
        will_flag: bool,
        clean_session: bool,
        keep_alive: u16
    },
    Connack {
        session_present: bool,
        return_code: ConnackReturnCode
    },
    Publish {
        topic_name: &'a str,
        packet_id: Option<PacketId>
    },
    Puback      { packet_id: PacketId },
    Pubrec      { packet_id: PacketId },
    Pubrel      { packet_id: PacketId },
    Pubcomp     { packet_id: PacketId },
    Subscribe   { packet_id: PacketId },
    Suback      { packet_id: PacketId },
    Unsubscribe { packet_id: PacketId },
    Unsuback    { packet_id: PacketId },
}
