extern crate clap;
use clap::Clap;
use std::str::FromStr;

#[derive(Clap, PartialEq, Debug)]
enum SortDimensions {
    Name,
    Type,
    Binding,
    NoSort
}

/// Utility that reads your i3 config file and prints a formatted version to the console.
#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Opts {
    /// Sets a custom config file. If not specified then the following paths will be checked (in order)
    /// ~/.config/i3/config, ~/.i3/config, /etc/i3/config
    #[clap(short, long)]
    config_path: Option<String>,
    /// whether to provide the output in csv (if not a table will be displayed)
    #[clap(long)]
    csv: bool,
    // whether to wait for input after printing the result (useful for executing in a volatile terminal)
    #[clap(short, long)]
    block: bool,
    // the dimension along which to sort the results
    #[clap(short, long, arg_enum, default_value = "binding")]
    sort_dim: SortDimensions
}

pub fn parse_cli_arguments() {
    // test command:
    // ./target/debug/i3-bindings -c "some other config.csad" -vv "potato man" test -d
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    dbg!(opts.config_path);
    dbg!(opts.csv);
    dbg!(opts.block);
    dbg!(opts.sort_dim);
}
