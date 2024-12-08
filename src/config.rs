use std::path::PathBuf;

use clap::{arg, command};

/// Configuration data for Pico.
pub struct Config {
    /// The paths to the PNG input files and directories.
    pub input_paths: Vec<PathBuf>,

    /// The path to the ICO output file.
    pub output_path: PathBuf,

    /// The resolution sizes to the ico file.
    pub resolution_sizes: Vec<i32>,

    /// Whether to sort ICO entries by descending resolution.
    pub sort: bool,

    /// Whether to overwrite an existing ICO output file.
    pub force: bool,
}

impl Config {
    /// Create a new config using command line arguments.
    pub fn new() -> Config {
        let args = command!()
            .arg(arg!(<input>... "One or more PNG input files or directories"))
            .arg(arg!(-o --output <path> "ICO output file"))
            .arg(arg!(-r --resolution <sizes> "ICO resolution sizes"))
            .arg(arg!(-s --sort "Sort ICO entries by resolution"))
            .arg(arg!(-f --force "Overwrite existing ICO output file"))
            .get_matches();

        let input_paths: Vec<PathBuf> = args
            .get_many::<String>("input")
            .unwrap()
            .map(PathBuf::from)
            .collect();

        let output_path = match args.get_one::<String>("output") {
            Some(path) => PathBuf::from(path),
            None => input_paths[0].with_extension("ico"),
        };
        // let parts: Vec<&str> = input_str.split(',').collect();

        let resolution_sizes = match args.get_one::<String>("resolution") {
            Some(size) => size
                .split(',')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect(),
            None => vec![0],
        };

        Config {
            input_paths,
            output_path,
            resolution_sizes,
            sort: args.get_flag("sort"),
            force: args.get_flag("force"),
        }
    }
}
