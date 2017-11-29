% GROOM(1)
% Christopher R. Field
% November 2017

# NAME

groom - A utility for processing mustache templates

# SYNOPSIS

groom [*options*] *input-file* [*output-file*]...

# DESCRIPTION

The Groom project is a Command-Line Interface (CLI) application for processing [Mustache](https://mustache.github.io/) templates. The project is primarily written in the [Rust](http://www.rust-lang.org) programming language. It can be installed on any [platform supported](https://forge.rust-lang.org/platform-support.html) by the Rust programming language, including Linux, macOS, and Windows. 

# OPTIONS

-d, \--debug
:   Changes the output stream for INFO, DEBUG, and TRACE log statements from `stdout` to `stderr`. This is useful for debugging templates without clutter up the `stdout` stream if redirecting to a file or piping to another application.

-m *FILE*, \--map=*FILE*
:   The YAML text data file that maps template tags (placeholders) to values. The default is to read from `stdin` unless this option is used to specify a file.

-v, \--verbose
:   Sets the level of verbosity. The higher the level of verbosity, the more log statements that are printed to either `stdout` or `stderr` depending on other flags and options. This flag can be specified multiple times, where each occurrence increases the level.

