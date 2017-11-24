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
        .arg(Arg::with_name("files")
             .help("Paths to files to be processed.")
             .takes_value(true)
             .multiple(true))
        .arg(Arg::with_name("debug")
             .help("Changes the output for INFO, DEBUG, and TRACE log statements from stdout to stderr.")
             .long("debug")
             .short("d"))
        .arg(Arg::with_name("output")
             .help("The output file for the compiled template. The default is to write the output to stdout.")
             .long("output")
             .short("o")
             .takes_value(true))
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
    .level(true)
    .init()
    .expect("logger to initiate");
    let result = Groom::new()
        .input(matches.values_of("input").and_then(|v| Some(v.collect())))
        .output(matches.value_of("output"))
        .run();
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
