// Copyright (C) 2017 Christopher R. Field.
//
// This file is part of Groom.
//
// Panser is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// Panser is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with Panser.  If not, see <http://www.gnu.org/licenses/>.

use std::env;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn exe_path() -> PathBuf {
    Path::new(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable"))
        .join("target")
        .join("debug")
        .join(&env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME environment variable"))
}

fn data_path() -> PathBuf {
    Path::new(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable"))
        .join("tests")
        .join("data")
}

#[test]
fn it_works() {
    let template = data_path().join("greeting.mustache");
    let process = Command::new(exe_path())
        .arg(template)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all("name: Chris".as_bytes()).expect("Write to stdin");
    let mut buf = String::new();
    process.stdout.expect("stdout").read_to_string(&mut buf).expect("Read from stdout");
    assert_eq!(&buf, "Hello Chris!\n");
}

