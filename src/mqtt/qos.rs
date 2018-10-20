pub enum QualityOfService {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce
}

impl serde::Serde for QualityOfService {
    fn ser(&self) -> Vec<u8> {
        match self {
            QualityOfService::AtMostOnce => 0_u8,
            QualityOfService::AtLeastOnce => 1_u8,
            QualityOfService::ExactlyOnce => 2_u8
        }
    }

    fn de<&'a>(bytes: &mut 'a[u8]) -> Result<(QualityOfService, u32), &'a str> {
        match byte {
            0 => Ok(QualityOfService::AtMostOnce),
            1 => Ok(QualityOfService::AtLeastOnce),
            2 => Ok(QualityOfService::ExactlyOnce),
            _ => Err("qos must be 0, 1, or 2")
        }
    }
}
