extern crate regex;
use crate::cli::AppOptions;
use regex::Regex;
use std::collections::HashMap;
use std::{env, fs, io, path};

#[derive(Debug)]
pub struct I3Binding {
    category: String,
    binding_type: String,
    binding: String,
    command: String,
}

pub fn read_config(opt: &AppOptions) -> Vec<I3Binding> {
    let home_dir_path = dirs::home_dir().unwrap();
    let home_dir = home_dir_path.to_str().unwrap();

    let mut config_files = vec![
        format!("{}/.config/i3/config", home_dir),
        format!("{}/.i3/config", home_dir),
        "/etc/i3/config".to_string(),
    ];

    // set custom config file path if provided as the only possible file path
    if let Some(cc) = &opt.config_path {
        config_files = vec![cc.clone()];
    }

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

    let raw_contents =
        fs::read_to_string(config_file_to_use).expect("Something went wrong when reading the file");

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
    let mut bindings: Vec<I3Binding> = Vec::new();

    for line in filtered_lines {
        if capture_category.is_match(line) {
            let caps = capture_category.captures(line).unwrap();
            last_category = caps.name("category").unwrap().as_str();
        } else if capture_exec.is_match(line) {
            let caps = capture_exec.captures(line).unwrap();
            let binding_type = match caps.name("binding_type").unwrap().as_str() {
                "bindsym" => "Symbol",
                "bindcode" => "Code",
                _ => "Unknown",
            };
            bindings.push(I3Binding {
                category: last_category.to_string(),
                binding_type: binding_type.to_string(),
                binding: caps.name("binding").unwrap().as_str().to_string(),
                command: caps.name("command").unwrap().as_str().to_string(),
            });
        }
    }

    bindings
}
