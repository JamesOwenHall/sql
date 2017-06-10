use std::collections::HashMap;
use data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    fields: HashMap<String, Data>,
}

impl Row {
    pub fn new() -> Self {
        Row{fields: HashMap::new()}
    }

    pub fn with_fields(fields: HashMap<String, Data>) -> Self {
        Row{fields: fields}
    }

    pub fn value(&self, column: &str) -> Option<Data> {
        self.fields.get(column).cloned()
    }
}
