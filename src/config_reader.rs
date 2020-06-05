extern crate regex;
use crate::cli::AppOptions;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs, io, path};

#[derive(Debug)]
pub struct I3Binding {
    category: String,
    binding_type: String,
    binding: String,
    command: String,
}

fn get_proper_config_file(opt: &AppOptions) -> PathBuf {
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

    existing_files
        .get(0)
        .expect("There is no usable config file")
        .canonicalize()
        .unwrap()
}

fn get_filtered_config_file_contents(file_path: PathBuf) -> Vec<String> {
    let raw_contents =
        fs::read_to_string(file_path).expect("Something went wrong when reading the file");
    let re = Regex::new(r"^\s*(bind|.*(C|c)ategory:)").unwrap();

    raw_contents
        .split("\n")
        .filter(|e| re.is_match(e))
        .map(|e: &str| e.trim().to_string())
        .collect()
}

fn get_list_of_i3bindings_from_content(content: Vec<String>) -> Vec<I3Binding> {
    let mut last_category = "default";
    let mut bindings: Vec<I3Binding> = Vec::new();

    // TODO: "Exec" modifiers are currently being added as part of the command
    let capture_exec =
        Regex::new(r"(?P<binding_type>bind\w+) (?P<binding>(\w|\$|\+)+) (exec )?(?P<command>.+)")
            .unwrap();
    let capture_category = Regex::new(r".*(C|c)ategory: (?P<category>.+)").unwrap();

    for line in &content {
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

pub fn read_config(opt: &AppOptions) -> Vec<I3Binding> {
    let config_file_to_use = get_proper_config_file(&opt);
    let mut filtered_lines = get_filtered_config_file_contents(config_file_to_use);

    get_list_of_i3bindings_from_content(filtered_lines)
}
