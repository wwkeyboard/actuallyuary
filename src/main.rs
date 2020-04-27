use clap::{App, Arg, SubCommand};
use std::env;
use std::io;

fn main() {
    let matches = App::new("myapp")
        .version("0.1")
        .about("Finds duplicate files.")
        .author("Aaron Lee <aaron@aaronosaur.us>")
        .arg(
            Arg::with_name("dir")
                .short("d")
                .takes_value(true)
                .help("root directory to start scanning"),
        )
        .subcommand(SubCommand::with_name("list").about("list files in this dir, for testing"))
        .get_matches();

    let directory = matches.value_of("dir");

    if matches.subcommand_matches("list").is_some() {
        list_directory(directory.unwrap_or("./").to_string()).unwrap();
    }
}

fn list_directory(path: String) -> Result<(), io::Error> {
    Ok(())
}
