use std::io::{Error, ErrorKind, Read, Result, Write};
use mqtt::*;

// https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Figure_2.2_-
// https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Table_2.4_Size
#[derive(Copy, Clone)]
pub struct FixedHeader {
    pub control_packet_type: ControlPacketType,
    pub flags: [bool; 4],
    pub remaining_length: u32, // actually a lower-bounded type encoded in up to 4 bytes
}

impl Serde for FixedHeader {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        let ctrl_bits = self.control_packet_type.to_byte();
        let ctrl_bits_written = sink.write(&[ctrl_bits])?;

        let flag_bits = self.flags
            .iter()
            .enumerate()
            .fold(0_u8, |acc, (idx, bit)| { acc + (*bit as u8) << (4_u8 - idx as u8) });
        let flag_bits_written = sink.write(&[flag_bits])?;

        let remaining_length_bytes =
            RemainingLength::encode(self.remaining_length)
            .map(|r| { r.bytes() })?;
        let rem_len_written = sink.write(&remaining_length_bytes)?;

        Ok(rem_len_written + ctrl_bits_written + flag_bits_written)
    }

    fn de(source: &mut Read) -> Result<(Self, usize)> {
        Err(Error::new(ErrorKind::Other, "not implemented"))
    }
}
