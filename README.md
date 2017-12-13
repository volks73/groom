# Groom: A command line application for processing Mustache templates

Groom is a Command-Line Interface (CLI) application for processing [Mustache](https://mustache.github.io/) templates. The project is primarily written in the [Rust](http://www.rust-lang.org) programming language. It can be installed on any [platform supported](https://forge.rust-lang.org/platform-support.html) by the Rust programming language, including Linux, macOS, and Windows. 

[![GitHub release](https://img.shields.io/github/release/volks73/groom.svg)](https://github.com/volks73/groom/releases)
[![license](https://img.shields.io/github/license/volks73/groom.svg)](https://github.com/volks73/groom/blob/master/LICENSE)

[Installation](#installation) | [Usage](#usage) | [Manual](https://volks73.github.io/groom/manpage.html) | [API](https://volks73.github.io/groom) | [Build](#build)

## Installation

Groom can be installed on any platform supported by the Rust programming language, including Linux, macOS, and Windows. It is possible to run Groom on Windows using the native command prompt (cmd.exe) or a terminal emulator, like [Mintty](https://mintty.github.io/) via [Cygwin](https://www.cygwin.com/).

### Windows

An installer (msi) with a pre-compiled binary is available with each [release](https://github.com/volks73/groom/releases). The installer will also add the installation location to the PATH system environment variable so the `groom` command can be executed from anywhere. Run the installer and follow the on-screen dialog to complete the installation.

It is also possible to install the application from source using Cargo. See the instructions for [installation via Cargo](#source) and use a command prompt (cmd.exe) or terminal emulator to execute the commands.

### macOS

Follow the instructions for [installation from source](#source).

### Linux

Follow the instructions for [installation from source](#source).

### Source

Download and install the following dependencies before installing the binary using Cargo.

- [Cargo](https://crates.io/), v0.17 or higher
- [Pandoc](http://pandoc.org), v1.19 or higher, optional
- [Rust](https://www.rust-lang.org/), v1.16 or higher

#### Application

Run the following commands from a terminal:

    $ git clone https://github.com/volks73/groom.git
    $ cd groom
    $ cargo install

Or obtain the source as an archive and run the following commands from a terminal:

    $ tar xf groom-#.#.#.tar.gz
    $ cd groom-#.#.#
    $ cargo install

where `#.#.#` is replaced with the version number of the source distribution, respectively. It might be desirable to change the install location by using the `--root` option with the `cargo install` command. See the `cargo install --help` for more information about installing a Rust binary crate using Cargo.

It might be desirable to change the install location by using the `--root` option with the `cargo install` command. See the `cargo install --help` for more information about installing a Rust binary crate using Cargo.

Note, if the groom binary was installed using cargo, then it can be uninstalled using `cargo uninstall groom`.

#### Documentation (Optional)

If the [Pandoc](http://pandoc.org) application was installed prior to installing from source via Cargo, i.e. `cargo install`, then a manpage in the [groff](https://www.gnu.org/software/groff/) format is automatically created from the [markdown](http://pandoc.org/MANUAL.html#pandocs-markdown) "source" file in the `man` directory using pandoc as part of the build script (`build.rs`). Otherwise, the manpage can be built with the following command:

    $ pandoc -s -t man -o man/groom.1 man/groom.1.md 

Regardless if the manpage (`groom.1`) was manually or automatically generated, it must be must be manually installed with the following command:

    $ mkdir -p ~/.cargo/share/man/man1
    $ cp man/groom.1 ~/.cargo/share/man/man1

If uninstalling groom using Cargo, i.e. `cargo uninstall groom`, then the manpage must also be manually removed as follows:

    $ rm ~/.cargo/share/man/man1/groom.1

## Usage

Process a template by reading data from `stdin` and rendering to `stdout`:

```bash
$ cat data.yml | groom template.mustache
```

or using redirection:

```bash
$ groom template.mustache < data.yml
```

or using the `-m,--map` option:

```bash
$ groom -m data.yml template.mustache
```

The template is rendered to `stdout` by default, but an optional `OUTPUT` argument can be used to specify a file for output instead of `stdout`:

```bash
$ groom -m data.yml template.mustache index.html
```

Note, the output file must be the second argument if it is used, but it is not required. The input template is required.

## Build

Download and install the same dependencies listed for [installing the application from source](#source), this includes the latest versions of [Rust](https://www.rust-lang.org), [Cargo](https://crates.io), and optionally [Pandoc](http://pandoc.org). 

### Application

Run the following commands from a terminal:

    $ git clone https://github.com/volks73/groom.git
    $ cd groom
    $ cargo build

Or obtain the source as an archive and run the following commands from a terminal:

    $ tar xf groom-#.#.#.tar.gz
    $ cd groom-#.#.#
    $ cargo build

where `#.#.#` is replaced with the version number of the source distribution, respectively. The `--release` flag can be added to the cargo command to build a release application instead of a debug application. 

### Documentation

Documentation is available in two forms: (i) [API](#api) and (ii) [Manpage](#manpage). The API documentation is for the library/crate while the Manpage documentation is helpful for the executable/binary. 

#### [API](https://volks73.github.io/groom)

Obtain the appropriate source and run the following commands from the root directory of the project in a terminal:

    $ cargo doc --no-deps

The output will be available in the `target/doc` folder.

#### [Manpage](https://volks73.github.io/groom/manpage.html)

Obtain the appropriate source and run the following commands from the root directory of the project in a terminal to build the manpage in the [groff](https://www.gnu.org/software/groff/) and html formats:

    $ cargo build --release

Or,

    $ pandoc -s -t man -o man/groom.1 man/groom.1.md
    $ pandoc -s -t html -o manpage.html man/groom.1.md

When the `release` profile is used to build the binary, the manpage is automatically generated if pandoc is installed.

## License

The Groom project is licensed under the [GPL-3.0 license](https://www.gnu.org/licenses/gpl-3.0.en.html). See the [LICENSE](https://github.com/volks73/groom/blob/master/LICENSE) file for more information about licensing and copyright.

