use mqtt::*;
use std::io::{Error, ErrorKind, Result, Read, Write};
use std::convert::TryFrom;

#[derive(Copy, Clone)]
pub struct RemainingLength(u32);

impl RemainingLength {
    pub const MAX_SIZE: u32 = 268_435_455u32;

    fn overflow_error<T>() -> Result<T> {
        Err(Error::new(ErrorKind::InvalidData, "Out of range"))
    }
}

impl TryFrom<u32> for RemainingLength {
    type Error = std::io::Error;

    fn try_from(value: u32) -> Result<Self> {
        if value <= RemainingLength::MAX_SIZE {
            Ok(RemainingLength(value))
        } else {
            RemainingLength::overflow_error()
        }
    }
}

impl Into<u32> for RemainingLength {
    fn into(self) -> u32 {
        self.0
    }
}

impl Serde for RemainingLength {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        match *self {
            RemainingLength(0) => sink.write(&[0u8]),
            RemainingLength(value) if value <= RemainingLength::MAX_SIZE => {
                let mut output = Vec::<u8>::new();
                let mut x = value;
                while x > 0 {
                    let mut encoded: u8 = (x % 128) as u8;
                    x = x / 128;
                    if x > 0 {
                        encoded = encoded | 128u8;
                    }
                    output.push(encoded);
                }
                sink.write(output.as_slice())
            },
            _ => RemainingLength::overflow_error()
        }
    }

    fn de(source: &mut Read) -> Result<(Self, usize)> {
        let mut value = 0u32;
        let mut bytes_read = 032;
        let mut mult = 1u32;
        for b in source.bytes() {
            bytes_read += 1;
            if bytes_read == 5 {
                return Err(Error::new(ErrorKind::InvalidData, "Remaining length is too large"))
            } else {
                match b {
                    Ok(b) => {
                        mult *= 128u32;
                        let decoded = (b & 127u8) as u32 * mult;
                        value += decoded;
                        let final_byte = b & 128u8 == 0;
                        if final_byte {
                            break
                        }
                    },
                    Err(e) => return Err(e)
                }
            }
        }
        Ok((RemainingLength(value), bytes_read as usize))
    }
}

impl RemainingLength {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Err(Error::new(ErrorKind::InvalidData, "Unimplemented"))
    }

    pub fn size(&self) -> usize {
        let RemainingLength(n) = *self;
        if n < 128 {
            1
        } else if n < 16_384 {
            2
        } else if n < 2_097_152 {
            3
        } else {
            4
        }
    }
}
