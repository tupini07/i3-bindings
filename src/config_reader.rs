extern crate dirs;
extern crate regex;

use crate::cli::{AppOptions, SortDimensions};
use regex::Regex;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct I3Binding {
    pub category: String,
    pub binding_type: String,
    pub binding: String,
    pub runtype: String,
    pub command: String,
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
        .map(|e| Path::new(e))
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
        .split('\n')
        .filter(|e| re.is_match(e))
        .map(|e: &str| e.trim().to_string())
        .collect()
}

fn get_list_of_i3bindings_from_content(
    content: Vec<String>,
    opts: &AppOptions,
) -> HashMap<String, Vec<I3Binding>> {
    let mut last_category = "default";
    let mut bindings: Vec<I3Binding> = Vec::new();

    // Binding type modifiers being extracted and added to the table
    let capture_exec =
        Regex::new(r"(?P<binding_type>bind\w+) (?P<binding>(\w|\$|\+)+) (?:(--release) )?(?P<type>(mode|exec|split|focus|move))? (?P<command>.+)")
            .unwrap();
    let capture_category = Regex::new(r".*(C|c)ategory: (?P<category>.+)").unwrap();

    for line in &content {
        if capture_category.is_match(line) {
            let caps = capture_category.captures(line).unwrap();
            last_category = caps.name("category").unwrap().as_str();
        } else if capture_exec.is_match(line) {
            // if we're only interested in one category then skip everything that is not said category
            if let Some(exclusive_category) = &opts.exclusive_category {
                if exclusive_category != last_category {
                    continue;
                }
            }

            let caps = capture_exec.captures(line).unwrap();
            let binding_type = match caps.name("binding_type").unwrap().as_str() {
                "bindsym" => "Symbol",
                "bindcode" => "Code",
                _ => "Unknown",
            };
            let _run_type = match caps.name("type") {
                None => "",
                _ => caps.name("type").unwrap().as_str()
            };
            bindings.push(I3Binding {
                category: last_category.to_string(),
                binding_type: binding_type.to_string(),
                binding: caps.name("binding").unwrap().as_str().to_string(),
                runtype: _run_type.to_string(),
                command: caps.name("command").unwrap().as_str().to_string(),
            });
        }
    }

    let mut bindings_map: HashMap<String, Vec<I3Binding>> = HashMap::new();
    for e in bindings {
        if !bindings_map.contains_key(&e.category) {
            bindings_map.insert(e.category.clone(), Vec::new());
        }

        bindings_map.get_mut(&e.category).unwrap().push(e);
    }

    // sort based on options
    if opts.sort_dim != SortDimensions::NoSort {
        for (_k, bindings) in bindings_map.iter_mut() {
            bindings.sort_by(|a, b| match opts.sort_dim {
                SortDimensions::Type => a.binding_type.cmp(&b.binding_type),
                SortDimensions::Binding => a.binding.cmp(&b.binding),
                SortDimensions::Command => a.command.cmp(&b.command),
                _ => Ordering::Greater,
            });
        }
    }

    bindings_map
}

pub fn read_bindings(opt: &AppOptions) -> HashMap<String, Vec<I3Binding>> {
    let config_file_to_use = get_proper_config_file(&opt);
    let filtered_lines = get_filtered_config_file_contents(config_file_to_use);

    get_list_of_i3bindings_from_content(filtered_lines, &opt)
}
