// lib.rs
//
// vim: ft=rust sw=4 ts=4
//
// Rust wrapper for pcre regular expressions library
// see docu at http://www.pcre.org/ 
//
// Author: Kay-Uwe Kirstein
//

#![crate_id = "pcre#0.2"]
#![crate_type = "dylib"]
#![desc = "Rust bindings to PCRE regular expression library (http://www.pcre.org)"]
#![license = "BSD"]

extern crate collections;
extern crate libc;

use collections::enum_set::{EnumSet, CLike};

// low-level functions and structs are the raw module
pub mod raw;

// options for constructing regex
// ==============================
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
// ======================
pub struct Regex {
	comp: *raw::PcreCompiled,
	extra: *raw::PcreExtra
}

// methods for Regex
impl Regex {
	pub fn new(pattern: &str, options: EnumSet<Flag>) -> Regex
	{
		//let Flags(opts) = options;

		let comp = match raw::compile(pattern, raw::PCRE_NONE) {
			Some(c)	=> c,
			None	=> ::std::ptr::null()
		};
		let extra = if !options.contains_elem(NoJIT) {
			match raw::study(comp, raw::PCRE_STUDY_JIT_COMPILE) {
				Some(e)	=> e,
				None	=> ::std::ptr::null()
			}
		} else {
			::std::ptr::null()
		};

		Regex { comp: comp, extra: extra }
	}

	// exec(&self) -> Match
	pub fn exec(&self, subject: &str, match_count: uint) -> Match {

		//let &Regex(comp, extra) = self;
		
		let res = raw::exec(self.comp, self.extra, subject, 0, raw::PCRE_NONE, match_count);
		match res {
			raw::Match(num, vec)		=> Match { status: Success, subject: subject.to_owned(), num_matches: num as uint, index_matches: vec },
			raw::MoreMatches(_, vec)	=> Match { status: Success, subject: subject.to_owned(), num_matches: vec.len()/3, index_matches: vec },
			raw::NoMatch				=> Match { status: Nomatch, subject: ~"", num_matches: 0u, index_matches: vec![] },
			raw::Error(n)				=> Match { status: Error, subject: format!("Error code: {:i}", n), num_matches: 0u, index_matches: vec![] }
		}
	}
}

// implement unsafe destructor for underlying data objects
#[unsafe_destructor]
impl Drop for Regex {
	fn drop(&mut self) {
		use std::ptr;

		//println!("Destructor Regex called for {:?}", *self);

		raw::free_extra(self.extra);
		self.extra = ptr::null();
		raw::free_compiled(self.comp);
		self.comp = ptr::null();

		//println(" ..done!");
	}
}

// struct for match results
// ========================
#[deriving(Eq, Show)]
pub enum MatchStatus {
	Success,
	Nomatch,
	Error
}
// implement Eq trait for easy status comparison
//impl Eq for MatchStatus {
//	fn eq(&self, other: &MatchStatus) -> bool { (*self as int) == (*other as int) }
//}

pub struct Match {
	// status of match operation
	pub status: MatchStatus,

	// this is an owned copy for easy access to matching substrings
	pub subject: ~str,

	// the number of matched groups
	pub num_matches: uint,
	
	// the vector of substring indices is kept private
	index_matches: Vec<i32>
}
impl Match {
	pub fn get_substring(&self, num: uint) -> Option<~str> {

		// check index bounds
		if num > self.num_matches {
			return None
		} else {
			let (start, end) = (*self.index_matches.get(2*num) as uint, *self.index_matches.get(2*num+1) as uint);
			Some(self.subject.slice(start, end).into_owned())
		}
	}

	pub fn get_all_substring(&self) -> Vec<~str> {

		self.get_all_substring_from(0)
	}

	pub fn get_all_substring_from(&self, from: uint) -> Vec<~str> {

		let mut substrings: Vec<~str> = Vec::with_capacity(self.num_matches);
		for i in range(from, self.num_matches) {
			let (start, end) = (*self.index_matches.get(2*i) as uint, *self.index_matches.get(2*i+1) as uint);
			substrings.push(self.subject.slice(start, end).into_owned());
		}
		
		substrings
	}
}

