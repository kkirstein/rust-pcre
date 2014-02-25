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

	// exec(&self) -> Match
	pub fn exec(&self, subject: &str, match_count: uint) -> Match {

		let &Regex(comp, extra) = self;
		
		let res = raw::exec(comp, extra, subject, 0, raw::PCRE_NONE, match_count);
		let (num_match, ind_match) = match res {
			raw::Match(num, vec)		=> (num as uint, vec),
			raw::MoreMatches(_, vec)	=> (vec.len()/3, vec),
			raw::NoMatch				=> return Match {
											status: Nomatch,
											subject: ~"",
											num_matches: 0u,
											index_matches: ~[] },
			raw::Error(n)				=> return Match {
											status: Error,
											subject: format!("Error code: {:i}", n),
											num_matches: 0u,
											index_matches: ~[] },
			//_						=> return Match { subject: ~"", num_matches: 0, index_matches: ~[] }
		};


		Match { status: Success, subject: subject.to_owned(), num_matches: num_match, index_matches: ind_match }
	}
}

// struct for match results
pub enum MatchStatus {
	Success,
	Nomatch,
	Error
}
// implement Eq trait for easy status comparison
impl Eq for MatchStatus {
	fn eq(&self, other: &MatchStatus) -> bool { (*self as int) == (*other as int) }
}

pub struct Match {
	// status of match operation
	status: MatchStatus,

	// this is an owned copy for easy access to matching substrings
	subject: ~str,

	// the number of matched groups
	num_matches: uint,
	
	// the vector of substring indices is kept private
	priv index_matches: ~[i32]
}

