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

extern mod extra;

use extra::enum_set::{EnumSet, CLike};
//use raw;
//use std::libc::{c_void, c_char, c_int};
//use std::str::raw::from_c_str;
//use std::ptr::is_not_null;
//use std::vec;

// low-level functions and structs are the raw module
pub mod raw;

// options for constructing regex
pub enum Flag {
	NoFlag			= 0x0000,
	CaseInsensitive = 0x0001,
	Multiline		= 0x0002,
	NoJIT 			= 0x0004
}
impl CLike for Flag {
	fn to_uint(&self) -> uint {
		*self as uint
	}

	fn from_uint(val: uint) -> Flag {
		match val {
			1	=> CaseInsensitive,
			2	=> Multiline,
			4	=> NoJIT,
			_	=> NoFlag
		}
	}
}

// basic struct for regex
pub struct Regex(*raw::PcreCompiled, *raw::PcreExtra);

// methods for Regex
impl Regex {
	pub fn new(pattern: &str, options: EnumSet<Flag>) -> Regex
	{
		//let Flags(opts) = options;

		let comp = match raw::compile(pattern, raw::PCRE_NONE) {
			Some(c)	=> c,
			None	=> ::std::ptr::null()
		};
		let extra = if (!options.contains_elem(NoJIT)) {
			match raw::study(comp, raw::PCRE_STUDY_JIT_COMPILE) {
				Some(e)	=> e,
				None	=> ::std::ptr::null()
			}
		} else {
			::std::ptr::null()
		};

		Regex(comp, extra)
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

