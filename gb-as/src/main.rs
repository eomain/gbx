
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

fn assemble(source: &str, output: &str, format: Format)
{
    let mut f = match File::open(source) {
        Err(e) => {
            eprintln!("error: {}: {}", source, e);
            return
        },
        Ok(f) => f
    };

    let mut input = String::new();
    f.read_to_string(&mut input);

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
            return
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
                 .help("Specify the output filename"))
        .arg(Arg::with_name("format")
                 .short("f")
                 .long("format")
                 .value_name("FORMAT")
                 .possible_values(&["bin"])
                 .takes_value(true)
                 .hide_possible_values(false)
                 .help("Output in specified format"));

    let m = app.clone().get_matches();
    let mut format = Format::Bin;
    let input = m.value_of("INPUT").unwrap();
    let mut output = String::from("out.gb");
    let name = m.is_present("output");
    if name {
        output = m.value_of("output").unwrap().into();
    }

    assemble(input, &output, format);
}
