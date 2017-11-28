# Groom: A command line application for processing Mustache templates #

[About](#what-is-groom) | [Installation](#installation) | [Build](#build) | [Examples](#examples)  

## What is Groom? ##

The Groom project is a Command-Line Interface (CLI) application for processing [Mustache](https://mustache.github.io/) templates. The project is primarily written in the [Rust](http://www.rust-lang.org) programming language. It can be installed on any [platform supported](https://forge.rust-lang.org/platform-support.html) by the Rust programming language, including Linux, macOS, and Windows. 

## Installation ##

Groom can be installed on any platform supported by the Rust programming language, including Linux, macOS, and Windows. It is possible to run Groom on Windows using the native command prompt (cmd.exe) or a terminal emulator, like [Mintty](https://mintty.github.io/) via [Cygwin](https://www.cygwin.com/).

### Windows ###

An installer (msi) with a pre-compiled binary is available with each [release](https://github.com/volks73/groom/releases). The installer will also add the installation location to the PATH system environment variable so groom can be executed from anywhere. Run the installer and follow the on-screen dialog to complete the installation.

It is also possible to install the application from source using Cargo. See the instructions for [installation via Cargo](#source) and use a command prompt (cmd.exe) or terminal emulator to execute the commands.

### macOS ###

Follow the instructions for [installation from source](#source).

### Linux ###

Follow the instructions for [installation from source](#source).

### Source ###

Download and install the following dependencies before installing the binary using Cargo.

- [Cargo](https://crates.io/), v0.17 or higher
- [Rust](https://www.rust-lang.org/), v1.16 or higher

Download and install the latest version of [Rust](https://www.rust-lang.org) before proceeding. [Cargo](https://crates.io) will be installed automatically with Rust.

#### Repository ####

Run the following commands from a terminal:

    $ git clone https://github.com/volks73/groom.git
    $ cd groom
    $ cargo install

It might be desirable to change the install location by using the `--root` option with the `cargo install` command. See the `cargo install --help` for more information about installing a Rust binary crate using Cargo.

#### Distribution ####

Obtain the appropriate source distribution as an archive file and run the following commands from a terminal:

    $ tar xf groom-#.#.#.tar.gz
    $ cd groom-#.#.#
    $ cargo install

where `#.#.#` is replaced with the version number of the source distribution, respectively. It might be desirable to change the install location by using the `--root` option with the `cargo install` command. See the `cargo install --help` for more information about installing a Rust binary crate using Cargo.

## Build ##

Download and install the same dependencies listed for [installing the application from source](#source), this includes the latest versions of [Rust](https://www.rust-lang.org), and [Cargo](https://crates.io). Run the following commands from a terminal:

    $ git clone https://github.com/volks73/groom.git
    $ cd groom
    $ cargo build

Or obtain the source as an archive and run the following commands from a terminal:

    $ tar xf groom-#.#.#.tar.gz
    $ cd groom-#.#.#
    $ cargo build

where `#.#.#` is replaced with the version number of the source distribution, respectively. The `--release` flag can be added to the cargo command to build a release application instead of a debug application. 

## Examples ##

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

## License ##

See the LICENSE file for more information about licensing and copyright.

## Contributors ##

See the AUTHORS file for information about contributors. Contributors are listed alphabetically by family name.

