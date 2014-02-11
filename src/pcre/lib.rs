// lib.rs
//
// vim: ft=rust sw=4 ts=4
//
// Rust wrapper for pcre regular expressions library
// see docu at http://www.pcre.org/ 
//
// Author: Kay-Uwe Kirstein
//

#[crate_id = "pcre#0.1"];
#[crate_type = "dylib"];
#[desc = "Rust bindings to PCRE regular expression library (http://www.pcre.org)"];
#[license = "BSD"];

//use raw;
//use std::libc::{c_void, c_char, c_int};
//use std::str::raw::from_c_str;
//use std::ptr::is_not_null;
//use std::vec;

// low-level functions and structs are the raw module
pub mod raw;

// options for constructing regex
pub enum Option {
	NoJIT = 0x0004
}
pub struct Options(uint);
impl Options {
	fn new(opts: ~[Option]) -> Options {
		Options(opts.iter().fold(0, |a, &b| a as uint | b as uint))
	}

	fn to_uint(&self) -> uint {
		let Options(val) = *self;
		val
	}
}

// basic struct for regex
pub struct Regex(*raw::PcreCompiled, *raw::PcreExtra);

// methods for Regex
impl Regex {
	fn new(pattern: &str, options: Options) -> Regex
	{
		// TODO: call raw::compile & raw::study
		Regex(::std::ptr::null(), ::std::ptr::null())
	}

	// TODO: match(&self) -> Match
}

// struct for match results
pub struct Match {

	// this is an owned copy for easy access to matching substrings
	subject: ~str,

	// the number of matched groups
	num_matches: uint,
	
	// the vector of substring indices is kept private
	priv index_matches: ~[uint]
}

