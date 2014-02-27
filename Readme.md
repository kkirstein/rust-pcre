# RUST-PCRE

Just another pcre-binding for the Rust programming language.

## Motivation

I started this project as a toy to get familar with the Rust programming language and its interface to existing (C-code) libraries.
[PCRE (Perl-Compatible-Regular Expressions)] (https://github.com/cadencemarseille/rust-pcre) was a good starting point for me, as I needed a regular expression library for Rust anyway and it is a small and clear library, easy to build on multiple targets.

For easy deployment, the build Rust library is self-contained, meaning it includes the PCRE code, which is build from the respective C-code as static lib and linked to the Rust lib.

## Status

Currently, only basic functionality of PCRE is supported, like compiling regular expressions, applying them to strings and extract matched substrings. For a more complete binding, you might consider more mature versions like [this one](https://github.com/cadencemarseille/rust-pcre).

This project includes pcre-8.33 and was successfully build on Windows7 (with MinGW/MSYS) and Ubuntu 13.10.

## Build

As rustpkg is currently deprecated ([ref](https://mail.mozilla.org/pipermail/rust-dev/2014-January/008224.html)), makefiles are used to build both PCRE and the Rust binding library.

Simply type 
```
make
```
to build the binding library (including PCRE) and
```
make test
```
to run additional tests.

## Usage

### Example

## ToDo

There several ideas for further development:
* Better error handling, e.g., return a Result<T, N> struct instead of an Option<T> from the raw functions
* Support more options of the underlying PCRE lib
* Implement more functionality, e.g. named matchgroups


