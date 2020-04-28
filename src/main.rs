use clap::{App, Arg, SubCommand};
//use std::env;
use std::fs::{self};
use std::io;
use std::path::{Path, PathBuf};

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
        .subcommand(
            SubCommand::with_name("one-file")
                .about("calculate checksum of one file")
                .arg(
                    Arg::with_name("filename")
                        .short("f")
                        .takes_value(true)
                        .help("the file to calculate"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        let directory = match matches.value_of("dir") {
            Some(dir) => Path::new(dir),
            None => Path::new("./"),
        };

        list_directory(directory.to_path_buf()).unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("one-file") {
        let filename = matches.value_of("filename").unwrap();
        println!("checking {}", filename);
    }
}

fn list_directory(dir: PathBuf) -> Result<(), io::Error> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().into_string().unwrap_or_default();
                if !name.starts_with(".") {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            list_directory(entry.path())?;
                        } else {
                            //                        if entry.file_name().
                            println!("{} => {:#?}", name, file_type.is_dir());
                        }
                    }
                }
            }
            //            let file = fs::File::open()?;
        }
        return Ok(());
    }
    Ok(())
}
