#[macro_use]
extern crate prettytable;
extern crate dirs;
extern crate regex;

use prettytable::{
    format::{self, TableFormat},
    Cell, Row, Table,
};
use regex::Regex;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;
use std::{env, fs, io, path};

fn main() {
    let home_dir_path = dirs::home_dir().unwrap();
    let home_dir = home_dir_path.to_str().unwrap();

    let config_files = vec![
        format!("{}/.config/i3/config", home_dir),
        format!("{}/.i3/config", home_dir),
        "/etc/i3/config".to_string(),
    ];

    let existing_files: Vec<_> = config_files
        .iter()
        .map(|e| path::Path::new(e))
        .filter(|&e| e.exists())
        .collect();

    let config_file_to_use = existing_files
        .get(0)
        .expect("There is no usable config file")
        .canonicalize()
        .unwrap();

    // read a file
    let file_path = dirs::home_dir()
        .unwrap()
        .join(path::Path::new(".config/i3/config"));

    let raw_contents = fs::read_to_string(config_file_to_use)
        .expect("Something went wrong when reading the file");

    #[derive(Debug)]
    struct I3Binding {
        binding_type: String,
        binding: String,
        command: String,
    }

    let re = Regex::new(r"^\s*(bind|.*(C|c)ategory:)").unwrap();

    // TODO: "Exec" modifiers are currently being added as part of the command
    let capture_exec =
        Regex::new(r"(?P<binding_type>bind\w+) (?P<binding>(\w|\$|\+)+) (exec )?(?P<command>.+)")
            .unwrap();
    let capture_category = Regex::new(r".*(C|c)ategory: (?P<category>.+)").unwrap();

    let mut filtered_lines: Vec<&str> = raw_contents
        .split("\n")
        .filter(|e| re.is_match(e))
        .map(|e: &str| e.trim())
        .collect();

    let mut last_category = "default";
    let mut bindings_map: HashMap<&str, Vec<I3Binding>> = HashMap::new();
    bindings_map.insert(last_category, Vec::new());

    for line in filtered_lines {
        if capture_category.is_match(line) {
            let caps = capture_category.captures(line).unwrap();
            last_category = caps.name("category").unwrap().as_str();

            if !bindings_map.contains_key(last_category) {
                bindings_map.insert(last_category, Vec::new());
            }
        } else if capture_exec.is_match(line) {
            let caps = capture_exec.captures(line).unwrap();
            let binding_type = match caps.name("binding_type").unwrap().as_str() {
                "bindsym" => "Symbol",
                "bindcode" => "Code",
                _ => "Unknown",
            };
            bindings_map
                .get_mut(last_category)
                .unwrap()
                .push(I3Binding {
                    binding_type: binding_type.to_string(),
                    binding: caps.name("binding").unwrap().as_str().to_string(),
                    command: caps.name("command").unwrap().as_str().to_string(),
                });
        }
    }

    let mut main_table = Table::new();
    main_table.set_titles(row!["Category", "Actual Binding"]);

    let mut sorted_vec: Vec<_> = bindings_map.iter_mut().collect();
    sorted_vec.sort_by(|a, b| a.0.cmp(b.0));

    for (&category, bindings_for_category) in sorted_vec.iter_mut() {
        if bindings_for_category.len() == 0 {
            continue;
        }

        let mut category_table = Table::new();

        bindings_for_category.sort_by(|e, i| {
            if e.binding > i.binding {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        for element in bindings_for_category.iter() {
            category_table.add_row(row![element.binding_type, element.binding, element.command]);
        }

        category_table.set_format(*prettytable::format::consts::FORMAT_CLEAN);

        main_table.add_row(row![category, category_table]);
    }

    main_table.set_format(
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
                &[format::LinePosition::Title, format::LinePosition::Intern],
                format::LineSeparator::new('─', '╫', '╟', '╢'),
            )
            .build(),
    );

    main_table.printstd();

    // wait for enter before exiting
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
