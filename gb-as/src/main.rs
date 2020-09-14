
extern crate clap;

mod token;
mod parse;
mod gen;

use std::{
    io::Read,
    fs::File
};
use clap::{
    App, AppSettings, Arg, ArgMatches,
    SubCommand as Command
};

/// The assembler output format
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Format {
    /// Raw binary output
    Bin
}

fn read_file(name: &str) -> Result<String, std::io::Error>
{
    let mut f = match File::open(name) {
        Err(e) => return Err((e)),
        Ok(f) => f
    };

    let mut input = String::new();
    f.read_to_string(&mut input);
    Ok(input)
}

fn read_file_token(name: &str) -> Result<Vec<token::Token>, ()>
{
    let mut input = match read_file(name) {
        Err(e) => return Err(()),
        Ok(input) => input
    };

    match token::scan(&input) {
        Err(_) => Err(()),
        Ok(tokens) => Ok(tokens)
    }
}

fn assemble(source: &str, output: &str, format: Format)
{
    let mut input = match read_file(source) {
        Err(e) => {
            eprintln!("error: {}: {}", source, e);
            return;
        },
        Ok(input) => input
    };

    let tokens = match token::scan(&input) {
        Err(_) => return,
        Ok(tokens) => tokens
    };

    let program = match parse::parse(tokens) {
        Err(_) => return,
        Ok(program) => program
    };

    let mut out = match File::create(output) {
        Err(e) => {
            eprintln!("error: {}: {}", output, e);
            return
        },
        Ok(f) => f
    };

    match gen::write(&mut out, &program) {
        Err(e) => {
            eprintln!("error: {}: {}", output, e);
            return;
        },
        Ok(_) => ()
    }
}

fn main()
{
    let mut app = App::new("gb-as")
        .about("Game Boy assembler")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("INPUT")
                 .required(true)
                 .index(1)
                 .help("Specify the source file to use"))
        .arg(Arg::with_name("output")
                 .short("o")
                 .value_name("FILE")
                 .default_value("out.bin")
                 .help("Specify the output filename"))
        .arg(Arg::with_name("format")
                 .short("f")
                 .long("format")
                 .value_name("FORMAT")
                 .default_value("bin")
                 .possible_values(&["bin"])
                 .takes_value(true)
                 .hide_possible_values(false)
                 .help("Output in specified format"));

    let matches = app.get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("output").unwrap();
    let format = match matches.value_of("format") {
        _ => Format::Bin
    };

    assemble(input, &output, format);
}
