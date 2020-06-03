// To print tables use this library
// https://github.com/phsym/prettytable-rs
#[macro_use]
extern crate prettytable;
extern crate dirs;
extern crate regex;

use prettytable::{Cell, Row, Table};
use regex::Regex;
use std::{env, fs, path};

fn main() {
    // Create the table
    let mut table = Table::new();

    // Add a row per time
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    // A more complicated way to add a row:
    table.add_row(Row::new(vec![
        Cell::new("foobar2"),
        Cell::new("bar2"),
        Cell::new("foo2"),
    ]));

    // Print the table to stdout
    table.printstd();

    // read a file
    let file_path = dirs::home_dir()
        .unwrap()
        .join(path::Path::new(".config/i3/config"));

    let raw_contents = fs::read_to_string(file_path.canonicalize().unwrap())
        .expect("Something went wrong when reading the file");

    #[derive(Debug)]
    struct I3Binding {
        binding: String,
        command: String,
    }

    let re = Regex::new(r"^\s*bind").unwrap();

    // TODO: "Exec" modifiers are currently being added as part of the command
    let capture_exec =
        Regex::new(r"bind\w+ (?P<binding>(\w|\$|\+)+) (exec )?(?P<command>.+)").unwrap();

    let filtered_contents: Vec<I3Binding> = raw_contents
        .split("\n")
        .filter(|e| re.is_match(e))
        .map(|e: &str| e.trim())
        .map(|e: &str| {
            let caps = capture_exec.captures(e).unwrap();

            I3Binding {
                binding: caps.name("binding").unwrap().as_str().to_string(),
                command: caps.name("command").unwrap().as_str().to_string(),
            }
        })
        .collect();

    // filtered_contents.iter()
    println!("file contents:\n{:?}", filtered_contents);
}
