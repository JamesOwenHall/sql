use std::collections::HashMap;
use data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    fields: HashMap<String, Data>,
}

impl Row {
    pub fn value(&self, column: &str) -> Option<Data> {
        self.fields.get(column).cloned()
    }
}
