use clap::{App, Arg, SubCommand};
//use std::env;
use std::fs::{self};
use std::io;
use std::path::Path;

fn main() {
    let matches = App::new("myapp")
        .version("0.1")
        .about("Finds duplicate files.")
        .author("Aaron Lee <aaron@aaronosaur.us>")
        .subcommand(
            SubCommand::with_name("list")
                .about("list files in this dir, for testing")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .takes_value(true)
                        .help("defaults to the current directory"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        let directory = match matches.value_of("dir") {
            Some(dir) => Path::new(dir),
            None => Path::new("./"),
        };

        list_directory(directory).unwrap();
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
