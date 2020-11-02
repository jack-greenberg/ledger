mod person;
mod receipt;
mod transaction;

use chrono::{Local, NaiveDate};
use clap::{crate_version, App, Arg};

#[macro_export]
macro_rules! today {
    () => {
        Local::now().date().naive_local()
    };
}

fn main() {
    let matches = App::new("Ledger")
        .version(crate_version!())
        .author("Jack Greenberg <j@jackgreenberg.co>")
        .about("Record transactions for housemates")
        .subcommand(
            App::new("receipt").arg(
                Arg::with_name("date")
                    .help("Date of the receipt")
                    .short("d")
                    .long("date")
                    .takes_value(true)
                    .value_name("DATE"),
            ),
        )
        .get_matches();

    // Create a new receipt with -d DATE
    if let Some(ref matches) = matches.subcommand_matches("receipt") {
        let date = match matches.value_of("date") {
            Some(date) => match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    eprintln!("Couldn't parse date!");
                    std::process::exit(1);
                }
            },

            // No date specified, so default to current date
            None => today!(),
        };
        println!("Creating file: {}", date.format("%Y-%m-%d.yml"))
    }
}
