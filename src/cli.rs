use clap::{ArgEnum, Parser};

#[derive(Copy, Clone, ArgEnum, PartialEq, Debug)]
pub enum SortDimensions {
    Command,
    Type,
    Binding,
    NoSort,
}

/// Utility that reads your i3 config file and prints a formatted version to the console.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct AppOptions {
    /// Sets a custom config file. If not specified then the following paths will be checked (in order)
    /// ~/.config/i3/config, ~/.i3/config, /etc/i3/config
    #[clap(short, long)]
    pub config_path: Option<String>,
    /// whether to provide the output in csv (if not a table will be displayed)
    #[clap(long)]
    pub csv: bool,
    /// define delimiter for csv
    #[clap(short, long, default_value = ",")]
    pub delimiter: char,
    /// whether to provide the markdow table
    #[clap(long)]
    pub md: bool,
    /// whether to wait for input after printing the result (useful for executing in a volatile terminal)
    #[clap(short, long)]
    pub block: bool,
    /// the dimension along which to sort the results
    #[clap(short, long, arg_enum, default_value = "binding")]
    pub sort_dim: SortDimensions,
    /// if specified then only bindings of this category will be shown
    #[clap(short, long)]
    pub exclusive_category: Option<String>,
    /// if specified then prints a table with the available categories
    #[clap(long)]
    pub print_categories: bool,
}

pub fn parse_cli_arguments() -> AppOptions {
    AppOptions::parse()

    // dbg!(&opts.config_path);
    // dbg!(&opts.csv);
    // dbg!(&opts.block);
    // dbg!(&opts.sort_dim);
    // dbg!(&opts.exclusive_category);
    // dbg!(&opts.print_categories);
}
