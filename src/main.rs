use clap::{crate_version, App, Arg, SubCommand};
mod receipt;
mod person;
mod transaction;

fn main() {
    let matches = App::new("Ledger")
        .version(crate_version!())
        .author("Jack Greenberg <j@jackgreenberg.co>")
        .about("Record transactions for housemates")
        .subcommand(
            App::new("person")
                .subcommand(
                    SubCommand::with_name("add")
                        .arg(
                            Arg::with_name("NAME")
                                .required(true)
                                .index(1)
                        ),
                )
        )
        .subcommand(
            App::new("transaction")
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("person") => {
            println!("Doing something with a person");
        },
        None => {
            eprintln!("Specify a person!");
        },
        _ => {
            unreachable!();
        }
    }
}
