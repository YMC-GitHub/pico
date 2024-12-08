mod config;
mod error;
mod icon;
mod image;
mod lang;

use std::{fs, path::PathBuf, process};

use config::Config;
use error::Error;
use ico_builder::IcoBuilder;
use icon::Icon;
use image::Image;

//  Result<(), Error>
//  Result<Image, Error>

/// Run Pico using command line arguments and exit on error.
fn main() {
    let config = Config::new();
    if let Err(error) = run_pico(&config) {
        eprintln!("{}", error);
        process::exit(1);
    }
}

/// Run Pico using configuration data.
fn run_pico(config: &Config) -> Result<(), Error> {
    if config.output_path.is_file() && !config.force {
        return Err(Error::OutputExists(config.output_path.clone()));
    }
    if config.resolution_sizes[0] > 0 {
        return png_to_ico(&config);
    }

    let paths = expand_paths(&config.input_paths)?;
    let images = read_images(paths)?;
    let data = Icon::from_images(images, config.sort).encode()?;
    fs::write(&config.output_path, data.as_slice())?;
    Ok(())
}

/// Expand a vector of paths to PNG files and directories to a vector of paths
/// to PNG files.
fn expand_paths(paths: &Vec<PathBuf>) -> Result<Vec<PathBuf>, Error> {
    let mut expanded = Vec::new();

    for path in paths {
        if path.is_dir() {
            expanded.append(&mut expand_dir(path)?);
        } else {
            expanded.push(path.clone());
        }
    }

    if expanded.is_empty() {
        Err(Error::NoInputs)
    } else {
        Ok(expanded)
    }
}

/// Expand a directory path to a vector of paths to PNG files.
fn expand_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut paths = Vec::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_file() && path.extension().unwrap_or_default().to_ascii_lowercase() == "png" {
            paths.push(path);
        }
    }

    Ok(paths)
}

/// Read a vector of images using a vector of paths to PNG input files.
fn read_images(paths: Vec<PathBuf>) -> Result<Vec<Image>, Error> {
    let mut images = Vec::with_capacity(paths.len());

    for path in paths {
        images.push(Image::from_path(path)?);
    }

    Ok(images)
}

fn png_to_ico(config: &Config) -> Result<(), Error> {
    let index = 0;
    let selected_path = &config.input_paths[index];
    let _ = IcoBuilder::default()
        .add_source_file(&selected_path)
        .build_file(&config.output_path);
    Ok(())
}
