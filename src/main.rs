use blake2::{Blake2b, Digest};
use clap::{App, Arg, SubCommand};
use sled;

use std::fs::{self, DirEntry};
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
                )
                .arg(
                    Arg::with_name("dbfile")
                        .short("f")
                        .takes_value(true)
                        .default_value("actu.db")
                        .help("filename of the database"),
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
                process_entry(entry)?;
            }
        }
        return Ok(());
    }
    Ok(())
}

// process_entry decides what to do with that entry,
// -- skips hidden files/dirs
// -- recurses if it's a directory
// -- handles other files
fn process_entry(entry: DirEntry) -> Result<(), io::Error> {
    let name = entry.file_name().into_string().unwrap();
    if !name.starts_with(".") {
        if let Ok(file_type) = entry.file_type() {
            // Skip symlinks for now, we don't want to get stuck in a loop.
            // TODO: if we can tell if the symlink is to a file instead of a directory maybe allow that
            if file_type.is_symlink() {
                return Ok(());
            }

            if file_type.is_dir() {
                list_directory(entry.path())?;
            } else {
                let checksum = checksum_for(entry.path())?;
                //record(checksum)?;
            }
        }
    };
    Ok(())
}

fn checksum_for(path: PathBuf) -> Result<Vec<u8>, io::Error> {
    println!("handling {}", path.to_str().unwrap());

    let mut file = fs::File::open(&path)?;
    let mut hasher = Blake2b::new();

    io::copy(&mut file, &mut hasher)?;

    Ok(hasher.result().as_slice().to_owned())
}
