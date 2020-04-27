use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::{self};
use std::io;
use std::path::Path;

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

    let directory = matches.value_of("dir").unwrap_or("./");

    let dir = Path::new(directory);

    if matches.subcommand_matches("list").is_some() {
        list_directory(dir).unwrap();
    }
}

fn list_directory(dir: &Path) -> Result<(), io::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            println!("{:#?}", entry);
        }
    }
    Ok(())
}
