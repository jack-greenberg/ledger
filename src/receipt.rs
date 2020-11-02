use chrono::{NaiveDate, Local};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate yaml_rust;
use crate::person::Person;
use yaml_rust::YamlLoader;

#[derive(Debug, Eq, PartialEq)]
struct Receipt {
    date: NaiveDate,
    buyer: Person,
    purpose: String,
    exclude: Vec<Person>,
}

impl Receipt {
    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        let template = format!(
            "date: {}\nbuyer:\npurpose:\nexclude:\n\n---\n",
            Local::now().format("%Y-%m-%d")
        );
        let path: &str = &self.date.format("receipts/%Y-%m-%d.yml").to_string();

        let mut file = File::create(path)?;
        file.write_all(template.as_bytes())?;
        Ok(())
    }

    pub fn from_file(path: String) -> Result<Receipt, Box<dyn Error>> {
        let mut file = File::open(&path)?;
        let mut receipt = String::new();

        file.read_to_string(&mut receipt)?;

        let yml = &YamlLoader::load_from_str(&receipt).unwrap()[0];

        let date = NaiveDate::parse_from_str(yml["date"].as_str().unwrap(), "%Y-%m-%d").unwrap();
        let buyer = Person {
            name: String::from(yml["buyer"].as_str().unwrap()),
            venmo: None,
        };
        let purpose = String::from(yml["purpose"].as_str().unwrap());
        let exclude = yml["exclude"]
            .as_vec()
            .unwrap()
            .iter()
            .map(|p| Person {
                name: String::from(p.as_str().unwrap()),
                venmo: None,
            })
            .collect();

        let r = Receipt {
            date,
            buyer, purpose,
            exclude,
        };

        Ok(r)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn receipt() {
        let r = Receipt::from_file(String::from("receipts/11-01-2020.yml")).unwrap();

        println!("{:?}", r);

        assert_eq!(
            r,
            Receipt {
                date: NaiveDate::parse_from_str("2020-11-01", "%Y-%m-%d").unwrap(),
                buyer: Person {
                    name: String::from("Nathan"),
                    venmo: None
                },
                purpose: String::from("Groceries"),
                exclude: vec!(Person {
                    name: String::from("Isabel"),
                    venmo: None
                }),
            }
        );
    }

    #[test]
    fn create() {}
}
