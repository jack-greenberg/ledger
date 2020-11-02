use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::person::Person;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    date: DateTime<Utc>,
    recipient: Person,
    sender: Person,
}

impl Record {
    pub fn new(recipient: Person, sender: Person) -> Self {
        Record {
            date: Utc::now(),
            recipient,
            sender,
        }
    }

    pub fn write_csv(self) -> Result<(), Box<dyn Error>> {
        let file_path = std::path::Path::new("test.csv");
        let mut writer = csv::Writer::from_path(file_path).unwrap();

        writer.serialize(self)?;
        Ok(())
    }
}
