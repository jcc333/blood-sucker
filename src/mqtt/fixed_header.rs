use std::io::{Read, Result, Write};
use mqtt::*;

// https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Figure_2.2_-
// https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Table_2.4_Size
trait FixedHeader : Serde {
    fn packet_type(&self) -> ControlPacketType;
    fn flags(&self) -> [bool; 4];
    fn remaining_length(&self) -> RemainingLength;
}

impl FixedHeader {
    fn to_first_byte(&self) -> u8 {
        let ctrl_bits = self.packet_type().to_byte();
        let flag_bits = self.flags()
            .iter()
            .enumerate()
            .fold(0u8, |acc, (idx, bit)| { acc + (*bit as u8) << (4u8 - idx as u8) });
        (ctrl_bits << 4) & flag_bits
    }

    fn from_first_byte(b: u8) -> Result<(ControlPacketType, [bool; 4])> {
        let ctrl_type = ControlPacketType::from_byte(b >> 4)?;
        let mut flags = [false; 4];
        for idx in 0..3 {
            flags[idx] = (b & (1u8 << idx)) != 0;
        }
        Ok((ctrl_type, flags))
    }
}

impl Serde for FixedHeader {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        let written = sink.write(&[self.to_first_byte()])?;
        let written = written + self.remaining_length.ser(sink)?;
        Ok(written)
    }

    fn de(source: &mut Read) -> Result<(Self, usize)> {
        let mut buf = [0; 1];
        source.read_exact(&mut buf)?;
        let (ctrl, flags)  = FixedHeader::from_first_byte(buf[0])?;
        let (remaining_length, remaining_length_size) = RemainingLength::de(source)?;
        let fixed_header = FixedHeader{
            packet_type: ctrl,
            flags,
            remaining_length
        };
        Ok((fixed_header, remaining_length_size + 1))
    }
}
