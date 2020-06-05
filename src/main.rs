#[macro_use]
extern crate prettytable;
extern crate dirs;

mod cli;
mod config_reader;

use prettytable::{
    format::{self, TableFormat},
    Cell, Row, Table,
};

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;
use std::{env, fs, io, path};

fn main() {
    let opts = cli::parse_cli_arguments();
    dbg!(config_reader::read_config(&opts));
    /////

    // let mut main_table = Table::new();
    // main_table.set_titles(row!["Category", "Actual Binding"]);
    //
    // let mut sorted_vec: Vec<_> = bindings_map.iter_mut().collect();
    // sorted_vec.sort_by(|a, b| a.0.cmp(b.0));
    //
    // for (&category, bindings_for_category) in sorted_vec.iter_mut() {
    //     if bindings_for_category.len() == 0 {
    //         continue;
    //     }
    //
    //     let mut category_table = Table::new();
    //
    //     bindings_for_category.sort_by(|e, i| {
    //         if e.binding > i.binding {
    //             Ordering::Greater
    //         } else {
    //             Ordering::Less
    //         }
    //     });
    //
    //     for element in bindings_for_category.iter() {
    //         category_table.add_row(row![element.binding_type, element.binding, element.command]);
    //     }
    //
    //     category_table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    //
    //     main_table.add_row(row![category, category_table]);
    // }
    //
    // main_table.set_format(
    //     format::FormatBuilder::new()
    //         .column_separator('║')
    //         .borders('║')
    //         .separators(
    //             &[format::LinePosition::Top],
    //             format::LineSeparator::new('═', '╦', '╔', '╗'),
    //         )
    //         .separators(
    //             &[format::LinePosition::Bottom],
    //             format::LineSeparator::new('═', '╩', '╚', '╝'),
    //         )
    //         .separators(
    //             &[format::LinePosition::Title, format::LinePosition::Intern],
    //             format::LineSeparator::new('─', '╫', '╟', '╢'),
    //         )
    //         .build(),
    // );
    //
    // main_table.printstd();

    // wait for enter before exiting
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
