#[macro_use]
extern crate prettytable;
extern crate dirs;
extern crate regex;

use prettytable::{
    format::{self, TableFormat},
    Cell, Row, Table,
};
use regex::Regex;
use std::cmp::Ordering;
use std::{env, fs, path};

fn main() {
    // read a file
    let file_path = dirs::home_dir()
        .unwrap()
        .join(path::Path::new(".config/i3/config"));

    let raw_contents = fs::read_to_string(file_path.canonicalize().unwrap())
        .expect("Something went wrong when reading the file");

    #[derive(Debug)]
    struct I3Binding {
        binding_type: String,
        binding: String,
        command: String,
    }

    let re = Regex::new(r"^\s*bind").unwrap();

    // TODO: "Exec" modifiers are currently being added as part of the command
    let capture_exec =
        Regex::new(r"(?P<binding_type>bind\w+) (?P<binding>(\w|\$|\+)+) (exec )?(?P<command>.+)")
            .unwrap();

    let mut filtered_contents: Vec<I3Binding> = raw_contents
        .split("\n")
        .filter(|e| re.is_match(e))
        .map(|e: &str| e.trim())
        .map(|e: &str| {
            let caps = capture_exec.captures(e).unwrap();
            let binding_type = match caps.name("binding_type").unwrap().as_str() {
                "bindsym" => "Symbol",
                "bindcode" => "Code",
                _ => "Unknown",
            };

            I3Binding {
                binding_type: binding_type.to_string(),
                binding: caps.name("binding").unwrap().as_str().to_string(),
                command: caps.name("command").unwrap().as_str().to_string(),
            }
        })
        .collect();

    filtered_contents.sort_by(|e, i| {
        if e.binding > i.binding {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    // Create the table
    let mut table = Table::new();

    // Add a row per time
    table.set_titles(row!["Type", "Binding", "Command"]);
    for xx in filtered_contents {
        table.add_row(row![xx.binding_type, xx.binding, xx.command]);
    }
    // table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_format(
        format::FormatBuilder::new()
            .column_separator('║')
            .borders('║')
            .separators(
                &[format::LinePosition::Top],
                format::LineSeparator::new('═', '╦', '╔', '╗'),
            )
            .separators(
                &[format::LinePosition::Bottom],
                format::LineSeparator::new('═', '╩', '╚', '╝'),
            )
            .separators(
                &[format::LinePosition::Title],
                format::LineSeparator::new('─', '╫', '╟', '╢'),
            )
            .build(),
    );

    // Print the table to stdout
    table.printstd();
}
