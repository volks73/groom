//! `groom` - A command line utility for processing [mustache](https://mustache.github.io/)
//! templates written in rust.

extern crate ansi_term;
extern crate atty;
#[macro_use] extern crate clap;
extern crate groom;
extern crate log;
extern crate loggerv;

use ansi_term::Colour;
use clap::{App, Arg};
use groom::Groom;
use log::LogLevel;
use loggerv::Output;
use std::error::Error;
use std::io::Write;

const ERROR_COLOR: Colour = Colour::Fixed(9); // bright red

fn main() {
    // Based on documentation for the ansi_term crate, Windows 10 supports ANSI escape characters,
    // but it must be enabled first. The ansi_term crate provides a function for enabling ANSI
    // support in Windows, but it is conditionally compiled and only exists for Windows builds. To
    // avoid build errors on non-windows platforms, a cfg guard should be put in place.
    #[cfg(windows)] ansi_term::enable_ansi_support().expect("Enable ANSI support on Windows");

    let matches = App::new("groom")
        .version(crate_version!())
        .about("An application for processing mustache templates")
        .arg(Arg::with_name("MAPPING")
             .help("The JSON text that maps template tags (placeholders) to values.")
             .index(1)
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("input")
             .help("The input source for the template. The default source is stdin.")
             .short("i")
             .long("input")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .help("The output destination for the processed template. The default destination is stdout.")
             .short("o")
             .long("output")
             .takes_value(true))
        .arg(Arg::with_name("debug")
             .help("Changes the output for INFO, DEBUG, and TRACE log statements from stdout to stderr.")
             .long("debug")
             .short("d"))
        .arg(Arg::with_name("verbose")
             .help("Sets the level of verbosity. The higher the level of verbosity, the more information that is printed and logged when the application is executed. This flag can be specified multiple times, where each occurrence increases the level.")
             .long("verbose")
             .short("v")
             .multiple(true))
        .get_matches();
    if matches.is_present("debug") {
        loggerv::Logger::new()
            .output(&LogLevel::Info, Output::Stderr)
            .output(&LogLevel::Debug, Output::Stderr)
            .output(&LogLevel::Trace, Output::Stderr)
    } else {
        loggerv::Logger::new()
    }.verbosity(matches.occurrences_of("verbose"))
    .module_path(false)
    .level(true)
    .init()
    .expect("logger to initiate");
    let result = Groom::new()
        .input(matches.value_of("input"))
        .output(matches.value_of("output"))
        .run(matches.value_of("MAPPING").unwrap());
    match result {
        Ok(_) => {
            std::process::exit(0);
        },
        Err(e) => {
            let mut tag = format!("Error[{}] ({})", e.code(), e.description());
            if atty::is(atty::Stream::Stderr) {
                tag = ERROR_COLOR.paint(tag).to_string()
            }
            writeln!(&mut std::io::stderr(), "{}: {}", tag, e)
                .expect("Writing to stderr");
            std::process::exit(e.code());
        }
    }
}
