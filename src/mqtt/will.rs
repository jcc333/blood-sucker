use mqtt::*;

/// A Will describes an optional message to be sent upon the
pub struct Will {
    pub retain: bool,
    pub qos: QualityOfService,
    pub topic: String,
    pub message: String
}
