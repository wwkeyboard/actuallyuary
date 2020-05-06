# actuallyuary

Scan all files in a directory(s), build a DB of their checksums, and
report back if there are any duplicates. Originally this was built to
check a series of backups to see if any of the photos in that backup
were duplicated. It might be useful to someone else so I'm going to
try and flesh it out into a better supported app. File an issue or
contact me if you see anything.

# Building

This is a Rust app that targets rust 2018. `rustup` is a great tool
for making sure you have a compatible compiler. Once you have a
working compiler check out the source and run `cargo build --release`
to build the app. The binary will be in `./target/release`.

`$ actuallyuary help` will print a description of the current
subcommands and their options.

# Development

To run the test suite `cargo test`. This suite doesn't require the
`./test_target` directory, that is a set of simple files for manual
testing.
