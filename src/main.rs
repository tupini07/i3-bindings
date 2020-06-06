#[macro_use]
extern crate prettytable;

mod cli;
mod config_reader;
mod drawers;
mod table_adapter;

use std::io;
use std::io::Read;

fn main() {
    let opts = cli::parse_cli_arguments();
    let bindings = config_reader::read_bindings(&opts);

    if opts.print_categories {
        println!("Categories: {:?}", bindings.keys());
    } else if opts.csv {
        drawers::csv_drawer::draw(bindings);
    } else {
        let table = table_adapter::build_table_from_bindings(bindings);
        drawers::table_drawer::draw(table);
    }

    if opts.block {
        // wait for enter before exiting
        let _ = io::stdin().read(&mut [0u8]).unwrap();
    }
}
