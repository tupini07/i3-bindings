#[macro_use]
extern crate prettytable;

mod cli;
mod config_reader;
mod drawers;
mod table_adapter;

use std::io::Read;
use std::io;

fn main() {
    let opts = cli::parse_cli_arguments();
    let bindings = config_reader::read_bindings(&opts);

    if opts.print_categories {
        println!("Categories: {:?}", bindings.keys());
    } else {
        let table = table_adapter::build_table_from_bindings(bindings);
        drawers::table_drawer::draw(table);
        // we can use prettytable to export to csv too!
        // https://github.com/phsym/prettytable-rs#user-content-csv-importexport
    }

    if opts.block {
        // wait for enter before exiting
        let _ = io::stdin().read(&mut [0u8]).unwrap();
    }
}
