use mqtt::*;
use std::io::{Error, ErrorKind, Read, Result, Write};

#[derive(Copy, Clone)]
pub enum QualityOfService {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce
}

impl QualityOfService {
    pub fn bits(&self) -> (bool, bool) {
        match *self {
            QualityOfService::AtMostOnce => (false, false),
            QualityOfService::AtLeastOnce => (false, true),
            QualityOfService::ExactlyOnce => (true, false)
        }
    }
}

impl Serde for QualityOfService {
    fn ser(&self, sink: &mut Write) -> Result<usize> {
        let byte = match &self {
            QualityOfService::AtMostOnce => 0u8,
            QualityOfService::AtLeastOnce => 1u8,
            QualityOfService::ExactlyOnce => 2u8,
        };
        sink.write(&[byte])
    }

    fn de(source: &mut Read) -> Result<(QualityOfService, usize)> {
        let mut buffer = [0u8; 1];
        let read = source.read(& mut buffer[..])?;
        match buffer[0] {
            0u8 => Ok((QualityOfService::AtMostOnce, read)),
            1u8 => Ok((QualityOfService::AtLeastOnce, read)),
            2u8 => Ok((QualityOfService::ExactlyOnce, read)),
            _ => Err(Error::new(ErrorKind::Other, "qos must be 0, 1, or 2"))
        }
    }
}
