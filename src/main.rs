use clap::{App, SubCommand};

fn main() {
    let matches = App::new("Ledger")
        .version("0.1.0")
        .author("Jack Greenberg <j@jackgreenberg.co>")
        .about("Record transactions for housemates")
        .subcommand(SubCommand::with_name("test"))
            .about("Do something")
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("test") {
        println!("Testing!");
    };
}
