use std::collections::HashMap;
use aggregate::AggregateCall;
use data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub fields: HashMap<String, Data>,
    pub aggregates: HashMap<AggregateCall, Data>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            fields: HashMap::new(),
            aggregates: HashMap::new(),
        }
    }
}
