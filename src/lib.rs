mod mqtt {
    mod qos;

    // https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Figure_2.1_-
    struct ControlPacket {
        fixed_header: FixedHeader,
        variable_header: Option<VariableHeader>,
        payload: Option<Payload>
    }

    struct VariableHeader {
        connect_flags: ConnectFlags,
        keep_alive: u16
    }

    struct WillFlags {
        retain: bool,
        qos: qos::QualityOfService,
    }

    // https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Figure_2.2_-
    struct FixedHeader {
        control_packet_type: ControlPacketType,
        flags: [bool; 4],
        remaining_length: Vec<u8> // serialized as per figure 2.2.3
    }

    // The prefix is the protocol_name + protocol_level: [0; 4; 'M'; 'Q'; 'T'; 'T'; 4; ]
    // protocol_name is always 6 bytes: [0; 4; 'M'; 'Q'; 'T'; 'T'] (4: u16, "MQTT")
    // protocol_level for 3.11 is 4: u8
    impl VariableHeader {
        fn as_bytes(self) -> [u8; 10] {
            let mut bytes: [u8; 10] = [0; 10];
            bytes[1] = 4_u8;
            bytes[2] = b'M';
            bytes[3] = b'Q';
            bytes[4] = b'T';
            bytes[5] = b'T';
            bytes[6] = 4_u8;
            bytes[7] = self.connect_flags.into();
            bytes[8] = (self.keep_alive >> 8_u16) as u8;
            bytes[9] = (self.keep_alive % (u8::max_value() as u16)) as u8;
            bytes
        }
    }

    impl From<WillFlags> for u8 {
        fn from(will_flags: WillFlags) -> Self {
            let retain_bit: u8 = ((will_flags.retain as u8) << 3_u8) as u8;
            let qos_bits: u8 = (u8::from(will_flags.qos) << 1_u8) as u8;
            let will_flag: u8 = true as u8;
            retain_bit | qos_bits | will_flag
        }
    }

}
