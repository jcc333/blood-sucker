pub type PacketId = Option<u16>;

pub enum ConnackReturnCode {
    /* 0 */    Accepted,
    /* 1 */    UnacceptableProtocolVersion,
    /* 2 */    IdentifierRejected,
    /* 3 */    ServerUnavailable,
    /* 4 */    BadUsernameOrPassword,
    /* 5 */    NotAuthorized,
}

pub enum VariableHeader<'a> {
    Connect {
        user_name: bool,
        password: bool,
        will_retain: bool,
        will_qos: bool,
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
        packet_id: PacketId
    },
    Puback      {  packet_id: PacketId },
    Pubrec      {  packet_id: PacketId },
    Pubrel      {  packet_id: PacketId },
    Pubcomp     {  packet_id: PacketId },
    Subscribe   {  packet_id: PacketId },
    Suback      {  packet_id: PacketId },
    Unsubscribe {  packet_id: PacketId },
    Unsuback    {  packet_id: PacketId },
}

// The prefix is the protocol_name + protocol_level: [0; 4; 'M'; 'Q'; 'T'; 'T']
// protocol_name is always 6 bytes: [0; 4; 'M'; 'Q'; 'T'; 'T'] (4: u16, "MQTT")
// protocol_level for 3.11 is `4: u8`
