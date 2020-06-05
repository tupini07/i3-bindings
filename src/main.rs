#[macro_use]
extern crate prettytable;
extern crate dirs;

mod cli;
mod config_reader;
mod drawers;
mod table_adapter;

use prettytable::{
    format::{self, TableFormat},
    Cell, Row, Table,
};

use crate::cli::AppOptions;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::io::Read;
use std::{env, fs, io, path};

fn main() {
    let opts = cli::parse_cli_arguments();
    let bindings = config_reader::read_bindings(&opts);
    let table = table_adapter::build_table_from_bindings(bindings);
    drawers::table_drawer::draw(table);
    // https://github.com/phsym/prettytable-rs#user-content-csv-importexport

    if opts.block {
        // wait for enter before exiting
        let _ = io::stdin().read(&mut [0u8]).unwrap();
    }
}
