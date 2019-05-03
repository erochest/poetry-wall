use std::path::PathBuf;
use std::str::FromStr;

use clap::{app_from_crate, Arg, ArgMatches, crate_authors, crate_description, crate_name,
           crate_version, value_t};
use palette::Srgb;

use poetry_wall::create_poetry_wall;
use poetry_wall::dimension::Dimension;
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
            Arg::with_name("max-font-size")
                .short("F")
                .long("max-font-size")
                .help("The size of type to use rendering the poem. If there's not enough room, \
                            it will be scaled down.")
                .value_name("TTF_FONT")
                .takes_value(true)
                .required(false)
                .default_value("72")
        )
        .arg(
            Arg::with_name("dimensions")
                .short("d")
                .long("dimensions")
                .help("The size of image to create, in the form 'WIDTHxHEIGHT'.")
                .value_name("DIMENSION")
                .takes_value(true)
                .required(false)
                .default_value("2880x2560")
        )
        .arg(
            Arg::with_name("left")
                .short("l")
                .long("left")
                .help("The size of the left margin. If omitted, it's computed.")
                .value_name("NUMBER")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("top")
                .short("t")
                .long("top")
                .help("The size of the top margin. If omitted, it's computed.")
                .value_name("NUMBER")
                .takes_value(true)
                .required(false)
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

    let poem_file: PathBuf = read_name_value(&matches, "poem")?;
    let font_file = read_name_value(&matches, "font")?;
    let output_file = read_name_value(&matches, "output")?;
    let color = color_name_value(&matches, "color")?;
    let background = color_name_value(&matches, "background")?;
    let font_size: f32 = read_name_value(&matches, "max-font-size")?;
    let dimensions: Dimension = read_name_value(&matches, "dimensions")?;
    let top: Option<u32> = matches.value_of("top").map(|v| v.parse()).transpose()?;
    let left: Option<u32> = matches.value_of("left").map(|v| v.parse()).transpose()?;

    Ok(PoetryWallOptions::new(
        poem_file,
        font_file,
        font_size,
        color,
        background,
        dimensions,
        top,
        left,
        output_file,
    ))
}

fn color_name_value(matches: &ArgMatches, name: &str) -> Result<Srgb<u8>> {
    matches
        .value_of(name)
        .ok_or_else(|| PoetryWallError::ColorError(None))
        .and_then(|color_name| palette::named::from_str(color_name)
            .ok_or_else(|| PoetryWallError::ColorError(Some(color_name.into()))))
}

fn read_name_value<T: FromStr>(matches: &ArgMatches, name: &str) -> Result<T> {
    value_t!(matches, name, T).map_err(|message| PoetryWallError::InvalidMissingOption(format!("{}: {}", &name, &message)))
}