use chrono::{DateTime, Utc};
use csv;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
}

impl Person {
    pub fn new(name: String) -> Self {
        Person { name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
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

#[cfg(test)]
mod test {
    use super::*;

    fn record() {
        let p1 = Person::new("Jack".to_string());
        let p2 = Person::new("Jill".to_string());
        let transaction = Record::new(p1, p2);
    }
}
