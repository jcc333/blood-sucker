use std::io::{Error, ErrorKind, Result};
use mqtt::*;

type TopicLength = u16;

pub enum Payload<'a> {
    Connect {
        client_id: &'a str,
        user_name: &'a str,
        password: &'a str,
        retain: bool,
        qos: QualityOfService,
        will: Option<(&'a str, &'a str)>,
        clean_session: bool,
        keep_alive: u16
    },
    Publish(&'a str),
    Subscribe(Vec<(TopicLength, &'a str, QualityOfService)>),
    Suback(Vec<Option<QualityOfService>>),
    Unsubscribe(Vec<Option<QualityOfService>>)
}

pub struct SubackReturn {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
    Failure
}

impl SubackReturn {
    fn from_qos(qos: Option<QualityOfService>) -> Self {
        match qos {
            None => Failure,
            Some(QualityOfService::AtMostOnce) => SubackReturn::AtMostOnce,
            Some(QualityOfService::AtLeastOnce) => SubackReturn::AtLeastOnce,
            Some(QualityOfService::ExactlyOnce) => SubackReturn::ExactlyOnce
        }
    }

    fn to_qos(&self) -> Option<QualityOfService> {
        Failure => None,
        SubAckReturn::AtMostOnce => Some(QualityOfService::AtMostOnce),
        SubAckReturn::AtLeastOnce => Some(QualityOfService::AtLeastOnce),
        SubAckReturn::ExactlyOnce => Some(QualityOfService::ExactlyOnce)
    }
    
    fn to_byte(&self) -> u8 {
        match self {
            SubackReturn::AtMostOnce => 0u8,
            SubackReturn::AtLeastOnce => 1u8,
            SubackReturn::ExactlyOnce => 2u8,
            SubackReturn::Failure => 128u8
        }
    }

    fn from_byte(byte: u8) -> Result<Self> {
        match byte {
            0u8 => Ok(SubackReturn::AtMostOnce),
            1u8 => Ok(SubackReturn::AtLeastOnce),
            2u8 => Ok(SubackReturn::ExactlyOnce),
            128u8 => Ok(SubackReturn::Failure),
            _ => Err(Error::new(
                    ErrorKind::InvalidData,
                    "suback return codes must be 0: at most once, 1: at least once, 2: exactly once, or 128: failure"
                ))

        }
    }
}
