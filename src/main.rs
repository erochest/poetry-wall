use std::path::PathBuf;

use clap::{app_from_crate, Arg, ArgMatches, crate_authors, crate_description, crate_name, crate_version};

use poetry_wall::create_poetry_wall;
use poetry_wall::error::{PoetryWallError, Result};
use poetry_wall::options::PoetryWallOptions;

fn main() -> Result<()> {
    let options = parse_options()?;
    Ok(())
}

fn parse_options() -> Result<PoetryWallOptions> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("poem")
                .short("p")
                .long("poem")
                .help("The poem to render in a markdown file.")
                .value_name("MARKDOWN_FILE")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("font")
                .short("f")
                .long("font")
                .help("The TTF font to use rendering the poem.")
                .value_name("TTF_FONT")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("The output file to be created as a PNG.")
                .value_name("PNG_FILE")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    let poem_file = path_buf_value(&matches, "poem")?;
    let font_file = path_buf_value(&matches, "font")?;
    let output_file = path_buf_value(&matches, "output")?;

    Ok(PoetryWallOptions::new(poem_file, font_file, output_file))
}

fn path_buf_value(matches: &ArgMatches, name: &str) -> Result<PathBuf> {
    let path_buf = matches
        .value_of(name)
        .ok_or_else(|| PoetryWallError::InvalidMissingOption(String::from(name)))?
        .into();
    Ok(path_buf)
}
