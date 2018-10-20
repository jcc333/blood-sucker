use mqtt::*;
mod mqtt;
fn foo() -> FixedHeader {
    FixedHeader{
        control_packet_type: ControlPacketType::Connect,
        flags: [false, false, false, false],
        remaining_length: 0u32
    }
}
