
extern crate gb_obj as obj;
extern crate clap;

mod token;
mod parse;
mod gen;

use crate::{
    parse::Table,
    parse::Program
};
use std::{
    io::Read,
    io::Write,
    fs::File,
    path::PathBuf,
    collections::HashSet
};
use obj::{
    Text,
    Section,
    Library
};
use clap::{
    App, AppSettings, Arg, ArgMatches,
    SubCommand as Command
};

/// The assembler output format
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Format {
    /// Raw binary output
    Bin,
    /// Library format
    Lib
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

fn create(output: &str) -> Result<File, ()>
{
    match File::create(output) {
        Err(e) => {
            eprintln!("error: {}: {}", output, e);
            Err(())
        },
        Ok(f) => Ok(f)
    }
}

fn path(name: &str) -> Result<PathBuf, ()>
{
    let mut path: PathBuf = name.into();
    if path.is_relative() {
        path = match std::env::current_dir() {
            Err(_) => return Err(()),
            Ok(path) => path
        };
        path.push(name);
    }
    Ok(path)
}

fn gen<W>(output: &str, w: &mut W, program: Program) -> Result<(), ()>
    where W: Write
{
    match gen::write(w, &program) {
        Err(e) => {
            eprintln!("error: {}: {}", output, e);
            Err(())
        },
        Ok(_) => Ok(())
    }
}

fn bin(output: &str, program: Program) -> Result<(), ()>
{
    let mut out = create(output)?;
    gen(output, &mut out, program);
    Ok(())
}

fn lib(output: &str, program: Program, table: Table) -> Result<(), ()>
{
    let mut bin = Vec::new();
    gen(output, &mut bin, program);

    let mut text = Text::new(bin);
    for (sym, addr) in table.iter() {
        text.sym(sym, *addr);
    }
    let sect = Section::new(text);
    let lib = Library::new(sect);
    let bin = match lib.write() {
        Err(_) => return Err(()),
        Ok(bin) => bin
    };

    let mut out = create(output)?;

    out.write(&bin);
    Ok(())
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

    let mut path: PathBuf = match path(source) {
        Err(_) => return,
        Ok(path) => path
    };

    let mut includes = HashSet::new();
    includes.insert(path);

    let (program, table) = match parse::parse(includes, tokens) {
        Err(_) => return,
        Ok((p, t)) => (p, t)
    };

    match format {
        Format::Bin => {
            bin(output, program);
        },

        Format::Lib => {
            lib(output, program, table);
        }
    }
}

fn main()
{
    let app = App::new("gb-as")
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
                 .possible_values(&["bin", "lib"])
                 .takes_value(true)
                 .hide_possible_values(false)
                 .help("Output in specified format"));

    let matches = app.get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("output").unwrap_or("out.bin");
    let format = match matches.value_of("format") {
        Some("lib") => Format::Lib,
        _ => Format::Bin
    };

    assemble(input, &output, format);
}
