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
pub enum Flag {
	CaseInsensitive = 0x0001,
	Multiline		= 0x0002,
	NoJIT 			= 0x0004
}
pub struct Flags(uint);
impl Flags {
	pub fn new(opts: ~[Flag]) -> Flags {
		Flags(opts.iter().fold(0, |a, &b| a as uint | b as uint))
	}

	pub fn to_uint(&self) -> uint {
		let Flags(val) = *self;
		val
	}
}

// basic struct for regex
pub struct Regex(*raw::PcreCompiled, *raw::PcreExtra);

// methods for Regex
impl Regex {
	pub fn new(pattern: &str, options:  Flags) -> Regex
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

