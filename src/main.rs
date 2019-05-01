use std::path::PathBuf;

use clap::{app_from_crate, Arg, ArgMatches, crate_authors, crate_description, crate_name, crate_version};
use palette::Srgb;

use poetry_wall::create_poetry_wall;
use poetry_wall::error::{PoetryWallError, Result};
use poetry_wall::options::PoetryWallOptions;

fn main() -> Result<()> {
    let options = parse_options()?;
    create_poetry_wall(&options)
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
            Arg::with_name("color")
                .short("c")
                .long("color")
                .help("The CSS name of the text color to use.")
                .value_name("CSS_COLOR_NAME")
                .takes_value(true)
                .required(false)
                .default_value("white")
        )
        .arg(
            Arg::with_name("background")
                .short("b")
                .long("background")
                .help("The CSS name of the background color to use.")
                .value_name("CSS_COLOR_NAME")
                .takes_value(true)
                .required(false)
                .default_value("black")
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
    let color = color_name_value(&matches, "color")?;
    let background = color_name_value(&matches, "background")?;

    Ok(PoetryWallOptions::new(poem_file, font_file, output_file, color, background))
}

fn path_buf_value(matches: &ArgMatches, name: &str) -> Result<PathBuf> {
    let path_buf = matches
        .value_of(name)
        .ok_or_else(|| PoetryWallError::InvalidMissingOption(String::from(name)))?
        .into();
    Ok(path_buf)
}

fn color_name_value(matches: &ArgMatches, name: &str) -> Result<Srgb<u8>> {
    matches
        .value_of(name)
        .ok_or_else(|| PoetryWallError::ColorError(None))
        .and_then(|color_name| palette::named::from_str(color_name)
            .ok_or_else(|| PoetryWallError::ColorError(Some(color_name.into()))))
}
