
extern crate clap;

mod format;

use clap::{
    App, Arg, ArgMatches,
    SubCommand as Command
};

fn main()
{
    let mut app = App::new("gbld")
        .about("Game Boy linker")
        .version(env!("CARGO_PKG_VERSION"));

    app.get_matches();
}
