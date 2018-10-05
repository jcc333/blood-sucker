mod mqtt {
    // https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Figure_2.1_-
    struct ControlPacket {
        fixed_header: FixedHeader,
        variable_header: Option<VariableHeader>,
        payload: Option<Payload>
    }
    
    struct VariableHeader {
        protocol_name: Vec<u8>,
        protocol_level: Any,
        connect_flags: Any,
        keep_alive: Any
    }

    struct Payload {
    }

    // https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Figure_2.2_-
    struct FixedHeader {
        control_packet_type: ControlPacketType,
        flags: [bool; 4],
        remaining_length: Vec<u8> // serialized as per figure 2.2.3
    }

        // https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Table_2.1_-
    enum ControlPacketType {
        ReservedLow,
        Connect,
        Connack,
        Publish,
        Puback,
        Pubrec,
        Pubrel,
        Pubcomp,
        Subscribe,
        Suback,
        Unsubscribe,
        Unsuback,
        Pingreq,
        Pingresp,
        Disconnect,
        ReservedHigh
    }

    enum QualityOfService {
        AtMostOnce,
        AtLeastOnce,
        ExactlyOnce
    }

    // https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Table_2.4_Size
    fn encode_remaining_length(len: u32) -> Result<Vec<u8>, String> {
        const MAX_SIZE: u32 = 268_435_455;
        match len {
            0 => Ok(vec![0x00]),
            _ if len > MAX_SIZE => Err("Out of range".to_string()),
            _  => {
                let mut output = Vec::<u8>::new();
                let mut x = len;
                while x > 0 {
                    let mut encoded: u8 = (x % 128) as u8;
                    x = x / 128;
                    if x > 0 {
                        encoded = encoded | 128u8;
                    }
                    output.push(encoded);
                }
                Ok(output)
            },
        }
    }

    fn decode_remaining_length(bytes: Vec<u8>) -> Result<u32, String> {
        if bytes.len() > 4 {
            Err("Too many bytes".to_string())
        } else {
            Ok(bytes.iter()
               .enumerate()
               .fold(0, |value, (idx, byte)| {
                   let multiplier = 128_u32.pow((idx + 1) as u32);
                   let byte_value = (byte & 127_u8) as u32;
                   (value as u32) + byte_value * multiplier
               }))
        }
    }

    impl ControlPacketType {
        fn flags(&self, dup: bool, qos: QualityOfService, retain: bool) -> [bool; 4] {
            return match self {
                ControlPacketType::Publish => {
                    let mut flags = [false, false, false, false];
                    flags[0] = dup;
                    match qos {
                        QualityOfService::AtMostOnce => {
                            flags[1] = false;
                            flags[2] = false;
                        },
                        QualityOfService::AtLeastOnce => {
                            flags[1] = false;
                            flags[2] = true;
                        },
                        QualityOfService::ExactlyOnce => {
                            flags[1] = true;
                            flags[2] = true;
                        }
                    }
                    flags[3] = retain;
                    flags
                }, 
                ControlPacketType::Pubrel => [false, false, true, false],
                ControlPacketType::Subscribe => [false, false, true, false],
                ControlPacketType::Unsubscribe => [false, false, true, false], 
                _ => [false, false, false, false]
            }
        }
    }
}
