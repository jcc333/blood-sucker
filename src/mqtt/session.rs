use mqtt::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::net::SocketAddr;
use std::io::{Error, ErrorKind, Result};

pub struct Session {
    filters: BTreeMap<QualityOfService, BTreeSet<String>>,
    will: Option<Will>
}

impl Session {
    fn new(will: Option<Will>) -> Self {
        let filters = BTreeMap::new();
        Session{ filters, will }
    }

    fn subscribe(&mut self, qos: QualityOfService, topic_filter: String) -> bool {
        self.filters
            .entry(qos).or_insert(BTreeSet::new())
            .insert(topic_filter)
    }
}

pub struct Sessions {
    sessions: HashMap<SocketAddr, Session>
}

impl Sessions {
    pub fn new() -> Self {
        Sessions{ sessions: HashMap::new() }
    }

    pub fn handle_message<'a>(&mut self, addr: &SocketAddr, msg: &'a mqtt::message::Message) -> Result<()> {
        match msg {
            Connect{
                client_id,
                username,
                password,
                will,
                clean_session,
                keep_alive
            } =>
                self.connect(
                    addr,
                    client_id,
                    username,
                    password,
                    will,
                    clean_session,
                    keep_alive
                ),
            Publish{ dup, qos, retain, topic, packet_id, payload } =>
                self.publish(addr, dup, qos, retain, topic, packet_id, payload),
            Puback{ packet_id } =>
                self.puback(addr, packet_id),
            Pubrec{ packet_id } =>
                self.pubrec(addr, packet_id),
            Pubrel{ packet_id } =>
                self.pubrel(addr, packet_id),
            Pubcomp{ packet_id } =>
                self.pubcomp(addr, packet_id),
            Subscribe{ packet_id, topic_filters } =>
                self.subscribe(addr, packet_id, topic_filters),
            Unsubscribe{ packet_id, topic_filters } =>
                self.unsubscribe(addr, packet_id, topic_filters),
            Pingreq{} =>
                self.pingreq(addr),
            Disconnect{} =>
                self.disconnect(addr),
            _ => Sessions::raise_wrong_direction()
        }
    }

    fn connect(&mut self,
                   addr: &SocketAddr,
                   client_id: String,
                   username: String,
                   password: String,
                   will: Option<Will>,
                   clean_session: bool,
                   keep_alive: u16
    ) -> Result<()> {
        println!("connect\t{}", addr.to_string());
        if self.sessions.contains_key(addr) {
            self.sessions.remove(addr);
            Sessions::raise_already_connected()
        } else {
            self.sessions.insert(addr.clone(), Session::new(will));
            Ok(())
        }
    }

    fn publish(&self,
               addr: &SocketAddr,
               dup: bool,
               qos: QualityOfService,
               retain: bool,
               topic: String,
               packet_id: Option<PacketId>,
               payload: String) -> Result<()> {
        println!("publish\t{}", addr.to_string());
        Ok(())
    }

    fn puback(&self, addr: &SocketAddr, packet_id: PacketId) -> Result<()> {
        println!("puback\t{}", addr.to_string());
        Ok(())
    }

    fn pubrec(&self, addr: &SocketAddr, packet_id: PacketId) -> Result<()> {
        println!("pubrec\t{}", addr.to_string());
        Ok(())
    }
    
    fn pubrel(&self, addr: &SocketAddr, packet_id: PacketId) -> Result<()> {
        println!("pubrel\t{}", addr.to_string());
        Ok(())
    }

    fn pubcomp(&self, addr: &SocketAddr, packet_id: PacketId) -> Result<()> {
        println!("pubcomp\t{}", addr.to_string());
        Ok(())
    }

    fn subscribe(
        &mut self,
        addr: &SocketAddr,
        packet_id: PacketId,
        topic_filters: Vec<(String, QualityOfService)>
    ) -> Result<()> {
        println!("subscribe\t{}", addr.to_string());
        Ok(())
    }

    fn unsubscribe(
        &mut self,
        addr: &SocketAddr,
        packet_id: PacketId,
        topic_filters: Vec<String>
    ) -> Result<()> {
        println!("unsubscribe\t{}", addr.to_string());
        Ok(())
    }

    fn pingreq(&self, addr: &SocketAddr) -> Result<()> {
        println!("pingreq\t{}", addr.to_string());
        Ok(())
    }

    fn disconnect(&mut self, addr: &SocketAddr) -> Result<()> {
        println!("disconnect\t{}", addr.to_string());
        self.sessions.remove(addr);
        Ok(())
    }

    fn raise_wrong_direction() -> Result<()> {
        Err(
            Error::new(
                ErrorKind::InvalidData,
                "received a server->client message as a server"
            )
        )
    }

    fn raise_already_connected() -> Result<()> {
        Err(
            Error::new(
                ErrorKind::InvalidData,
                "received a connect message from an already-connected address"
            )
        )
    }
}
