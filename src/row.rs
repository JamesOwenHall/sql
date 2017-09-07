use std::collections::HashMap;
use data::Data;
use expr::AggregateCall;

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    fields: HashMap<String, Data>,
    aggregates: HashMap<AggregateCall, Data>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            fields: HashMap::new(),
            aggregates: HashMap::new(),
        }
    }

    pub fn with_fields(fields: HashMap<String, Data>) -> Self {
        Row {
            fields: fields,
            aggregates: HashMap::new(),
        }
    }

    pub fn field_value(&self, column: &str) -> Option<Data> {
        self.fields.get(column).cloned()
    }

    pub fn aggregate_value(&self, call: &AggregateCall) -> Option<Data> {
        self.aggregates.get(call).cloned()
    }
}
