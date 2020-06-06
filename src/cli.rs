extern crate clap;
use clap::Clap;
use std::str::FromStr;

#[derive(Clap, PartialEq, Debug)]
pub enum SortDimensions {
    Command,
    Type,
    Binding,
    NoSort,
}

/// Utility that reads your i3 config file and prints a formatted version to the console.
#[derive(Clap)]
#[clap(version = "0.1.0")]
pub struct AppOptions {
    /// Sets a custom config file. If not specified then the following paths will be checked (in order)
    /// ~/.config/i3/config, ~/.i3/config, /etc/i3/config
    #[clap(short, long)]
    pub config_path: Option<String>,
    /// whether to provide the output in csv (if not a table will be displayed)
    #[clap(long)]
    pub csv: bool,
    // whether to wait for input after printing the result (useful for executing in a volatile terminal)
    #[clap(short, long)]
    pub block: bool,
    // the dimension along which to sort the results
    #[clap(short, long, arg_enum, default_value = "binding")]
    pub sort_dim: SortDimensions,
    // if specified then only bindings of this category will be shown
    #[clap(short, long)]
    pub exclusive_category: Option<String>,
    // if specified then prints a table with the available categories
    #[clap(long)]
    pub print_categories: bool,
}

pub fn parse_cli_arguments() -> AppOptions {
    // test command:
    // ./target/debug/i3-bindings -c "some other config.csad" -vv "potato man" test -d
    let opts: AppOptions = AppOptions::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    dbg!(&opts.config_path);
    dbg!(&opts.csv);
    dbg!(&opts.block);
    dbg!(&opts.sort_dim);
    dbg!(&opts.exclusive_category);
    dbg!(&opts.print_categories);

    opts
}
