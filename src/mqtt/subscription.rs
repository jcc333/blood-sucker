use mqtt::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub struct Session<'a> {
    filters: BTreeMap<QualityOfService, BTreeSet<&'a str>>
}

impl<'a> Session<'a> {
    fn subscribe(&mut self, qos: QualityOfService, topic_filter: &'a str) -> bool {
        self.filters
            .entry(qos).or_insert(BTreeSet::new())
            .insert(topic_filter)
    }
}
