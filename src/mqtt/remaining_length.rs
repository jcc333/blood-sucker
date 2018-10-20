use std::io::{Error, ErrorKind, Result};

pub enum RemainingLength {
    One(u8),
    Two(u8, u8),
    Three(u8, u8, u8),
    Four(u8, u8, u8, u8),
}

impl RemainingLength {
    const MAX_SIZE: u32 = 268_435_455;
    const TERMINAL_BYTE: u8 = 127;

    pub fn encode(value: u32) -> Result<Self> {
        if value == 0 {
            Ok(RemainingLength::One(0u8))
        } else if value <= RemainingLength::MAX_SIZE {
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
            match output.len() {
                1 => Ok(RemainingLength::One(output[0])),
                2 => Ok(RemainingLength::Two(output[0], output[1])),
                3 => Ok(RemainingLength::Three(output[0], output[1], output[2])),
                4 => Ok(RemainingLength::Four(output[0], output[1], output[2], output[3])),
                _ => Err(Error::new(ErrorKind::InvalidData, "Out of range"))
            }
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Out of range"))
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        match *self {
            RemainingLength::One(b0) => vec![b0],
            RemainingLength::Two(b0, b1) => vec![b0, b1],
            RemainingLength::Three(b0, b1, b2) => vec![b0, b1, b2],
            RemainingLength::Four(b0, b1, b2, b3) => vec![b0, b1, b2, b3]
        }
    }

    pub fn decode(&self) -> u32 {
        self.bytes()
            .iter()
            .enumerate()
            .fold(0, |value, (idx, byte)| {
                let multiplier = 128_u32.pow((idx + 1) as u32);
                let byte_value = (byte & RemainingLength::TERMINAL_BYTE) as u32;
                (value as u32) + byte_value * multiplier
            })
    }

    pub fn size(&self) -> usize {
        match *self {
            RemainingLength::One(_) => 1usize,
            RemainingLength::Two(_, _) => 2usize,
            RemainingLength::Three(_, _, _) => 3usize,
            RemainingLength::Four(_, _, _, _) => 4usize,
        }
    }
}
