use std::collections::HashMap;

use crate::core::data::RecordData;

#[derive(Debug, Clone)]
pub enum Command {
    /// add a record
    Record(RecordData),
    /// remove a record
    Remove(u32),
    // install via a source
    // Install(Source, Name),
}

#[derive(Debug, Default, Clone)]
pub struct Manager {
    index: HashMap<u32, RecordData>,
}

impl Manager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(data: Vec<RecordData>) -> Self {
        let mut index = HashMap::new();
        for rec in data {
            index.insert(rec.id, rec);
        }
        Self { index }
    }

            
}
