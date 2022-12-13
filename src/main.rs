// Copyright (C) 2017 Christopher R. Field.
//
// This file is part of Groom.
//
// Groom is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Groom is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Groom.  If not, see <http://www.gnu.org/licenses/>.

extern crate ansi_term;
extern crate atty;
#[macro_use]
extern crate clap;
extern crate groom;
extern crate log;
extern crate loggerv;

use ansi_term::Colour;
use clap::{App, Arg};
use groom::Groom;
use log::Level;
use loggerv::Output;
use std::io::Write;

const ERROR_COLOR: Colour = Colour::Fixed(9); // Bright red

fn main() {
    // Based on documentation for the ansi_term crate, Windows 10 supports ANSI escape characters,
    // but it must be enabled first. The ansi_term crate provides a function for enabling ANSI
    // support in Windows, but it is conditionally compiled and only exists for Windows builds. To
    // avoid build errors on non-windows platforms, a cfg guard should be put in place.
    #[cfg(windows)]
    ansi_term::enable_ansi_support().expect("Enable ANSI support on Windows");

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("map")
             .help("The YAML text data that maps template tags (placeholders) to values. The default is to read from stdin.")
             .short("m")
             .long("map")
             .takes_value(true))
        .arg(Arg::with_name("INPUT")
             .help("The input source template to render.")
             .index(1)
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("OUTPUT")
             .help("The output destination for the rendered template. The default destination is stdout.")
             .index(2))
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
    let verbosity = matches.occurrences_of("verbose");
    if matches.is_present("debug") {
        loggerv::Logger::new()
            .output(&Level::Info, Output::Stderr)
            .output(&Level::Debug, Output::Stderr)
            .output(&Level::Trace, Output::Stderr)
    } else {
        loggerv::Logger::new()
    }
    .verbosity(verbosity)
    .line_numbers(verbosity > 3)
    .module_path(false)
    .level(true)
    .init()
    .expect("logger to initiate");
    let result = Groom::new()
        .data(matches.value_of("map"))
        .output(matches.value_of("OUTPUT"))
        .run(matches.value_of("INPUT").unwrap());
    match result {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(e) => {
            let mut tag = format!("Error[{}] ({})", e.code(), e);
            if atty::is(atty::Stream::Stderr) {
                tag = ERROR_COLOR.paint(tag).to_string()
            }
            writeln!(&mut std::io::stderr(), "{}: {}", tag, e).expect("Writing to stderr");
            std::process::exit(e.code());
        }
    }
}
