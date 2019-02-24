use mqtt::*;

pub struct Connect {
    client_id: String,
    username: String,
    password: String,
    will: Option<Will>,
    clean_session: bool,
    keep_alive: u16,
}

impl Message for Connect {
    fn packet_type(&self) -> ControlPacketType{
        ControlPacketType::Connect
    }

    fn variable_header(&self) -> Option<VariableHeader> {
        let retain = self.will.map_or(false, |will| { will.retain });
        let qos = self.will.map_or(QualityOfService::AtMostOnce, |will| { will.qos });
        let flag = self.will.is_some();
        Some(VariableHeader::Connect {
            username: self.username.len() != 0,
            password: self.password.len() != 0,
            will_retain: retain,
            will_qos: qos,
            will_flag: flag,
            clean_session: self.clean_session,
            keep_alive: self.keep_alive
        })
    }

    fn payload(&self) -> Option<Payload> {
        let will_topic_by_msg = match self.will {
            Some(Will{ retain: _, qos: _, topic, message }) =>
                Some((topic.clone(), message.clone())),
            None => None
        };
        Some(Payload::Connect{
            client_id: &self.client_id,
            will: will_topic_by_msg,
            username: &self.username,
            password: &self.password
        })
    }
}
