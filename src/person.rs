use csv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Person {
    pub name: String,
    pub venmo: Option<String>,
}

impl Person {
    pub fn new(name: String, venmo: Option<String>) -> Self {
        Person { name, venmo }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::transaction::Record;

    #[test]
    fn record() {
        let p1 = Person::new("Jack".to_string(), Some("@greenberg-jack".to_string()));
        let p2 = Person::new("Jill".to_string(), Some("@jill".to_string()));

        let transaction = Record::new(p1, p2);
    }
}
