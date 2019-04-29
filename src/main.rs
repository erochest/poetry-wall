use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, ArgMatches, Arg};
use crate::error::PoetryWallError;

fn main() -> Result<(), PoetryWallError> {
    let _args = parse_args();
    Ok(())
}

fn parse_args<'a>() -> ArgMatches<'a> {
    app_from_crate!()
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
        .get_matches()
}

mod error;