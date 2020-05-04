use anyhow::{Context, Result};
use bincode;
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
                        .default_value(".actu.db")
                        .help("directory name of the database(if this is not hidden it could get included in future scans)"),
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

    match matches.subcommand() {
        ("list", Some(matches)) => {
            let directory = match matches.value_of("dir") {
                Some(dir) => Path::new(dir),
                None => Path::new("./"),
            };

            // dbfile has a default
            let dbfilename = matches.value_of("dbfile").unwrap();
            let db = sled::open(dbfilename).unwrap();

            list_directory(&db, directory.to_path_buf()).unwrap();

            db.flush().unwrap();
        }
        ("one-file", Some(matches)) => {
            let filename = matches.value_of("filename").unwrap();
            println!("checking {}", filename);
        }
        _ => println!("{}", matches.usage()),
    }
}

fn list_directory(db: &sled::Db, dir: PathBuf) -> Result<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                process_entry(&db, entry)?;
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
fn process_entry(db: &sled::Db, entry: DirEntry) -> Result<()> {
    let name = entry.file_name().into_string().unwrap();
    if !name.starts_with(".") {
        if let Ok(file_type) = entry.file_type() {
            // Skip symlinks for now, we don't want to get stuck in a loop.
            // TODO: if we can tell if the symlink is to a file instead of a directory maybe allow that
            if file_type.is_symlink() {
                return Ok(());
            }

            if file_type.is_dir() {
                list_directory(db, entry.path())?;
            } else {
                let filename = entry.path().to_str().unwrap().to_string();
                let checksum = checksum_for(&entry.path())?;

                let payload = match db
                    .get(&checksum)
                    .with_context(|| format!("inserting {}", filename))?
                {
                    // This checksum is already in the DB
                    Some(v) => {
                        let mut existing: Vec<String> = bincode::deserialize(&v)?;

                        // Only add the filename if it doesn't already exist in the DB
                        if existing.contains(&filename) {
                            if existing.len() > 1 {
                                let dups: Vec<String> = existing
                                    .iter()
                                    .filter_map(|s| {
                                        if **s != filename {
                                            Some(s.to_owned())
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                println!("{} matched\n  {}", filename, dups.join("\n  "));
                            }
                        } else {
                            println!("* {} matched\n  {}", filename, existing.join("\n  "));
                            existing.push(filename);
                        }
                        existing
                    }
                    // The checksum isn't in the DB
                    None => vec![filename],
                };

                let value = bincode::serialize(&payload).unwrap();

                db.insert(checksum, value)
                    .with_context(|| format!("inserting {}", entry.path().to_str().unwrap()))?;
            }
        }
    };
    Ok(())
}

fn checksum_for(path: &PathBuf) -> Result<Vec<u8>, io::Error> {
    let mut file = fs::File::open(&path)?;
    let mut hasher = Blake2b::new();

    io::copy(&mut file, &mut hasher)?;

    Ok(hasher.result().as_slice().to_owned())
}
