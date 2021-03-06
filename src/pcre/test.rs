// test.rs
//
// vim: ft=rust sw=4 ts=4
//
// Test case for PCRE Rust bindings
// see docu at http://www.pcre.org/ 
//
// Author: Kay-Uwe Kirstein
//

#![license = "BSD"]

extern crate collections;
extern crate pcre;

// tests for high-level API
// ========================
#[test]
fn test_new_flags() {
	use pcre::{CaseInsensitive, Multiline, NoJIT, Flag};
	use collections::enum_set::EnumSet;

	let mut opts: EnumSet<Flag> = EnumSet::empty();
	assert!(opts.is_empty());

	opts.add(CaseInsensitive);
	assert!(opts.contains_elem(CaseInsensitive));

	opts.add(NoJIT);
	assert!(opts.contains_elem(NoJIT) & opts.contains_elem(CaseInsensitive));

	opts.add(Multiline);
	assert!(opts.contains_elem(NoJIT) & opts.contains_elem(CaseInsensitive)
		   & opts.contains_elem(Multiline));
}

#[test]
#[ignore]
fn test_new_regex() {
	use pcre::{Flag, NoJIT, Regex};
	use collections::enum_set::EnumSet;

	let pat = "cat";
	let regex = Regex::new(pat, EnumSet::empty());
	// fields are now private, so can't be checked anymore
	match regex {
	//	Ok(r)	=> (),
	//	Err(n)	=> fail!(),
		_		=> ()
	}

	let mut opts:EnumSet<Flag> = EnumSet::empty();
	opts.add(NoJIT);
	let regex = Regex::new(pat, opts);
	// fields are now private, so can't be checked anymore
	match regex {
	//	Ok(r)	=> (),
	//	Err(n)	=> fail!(),
		_		=> ()
	}
}

#[test]
#[ignore]
fn test_new_match() {
	fail!();
}

#[test]
fn test_simple_match() {
	use pcre::{Regex, Success};
	use collections::enum_set::EnumSet;

	let regex = Regex::new("cat", EnumSet::empty());
	let subject = ~"dog and cat";
	let mm = regex.exec(subject, 1);

	assert_eq!(Success, mm.status);
	assert_eq!(subject, mm.subject);
}

#[test]
fn test_simple_match_substring() {
	use pcre::Regex;
	use collections::enum_set::EnumSet;

	let subject = "dog and cat";

	let regex = Regex::new("cat", EnumSet::empty());
	let mm = regex.exec(subject, 1);
	let substring = mm.get_substring(0);

	match substring {
		None	=> fail!(),
		Some(s)	=> assert_eq!(~"cat", s)
	}
}

#[test]
fn test_simple_match_multi_substring() {
	use pcre::Regex;
	use collections::enum_set::EnumSet;

	let subject = "00:06:08 TAG_1      id=   3,   320,     1,  -321,    11, TAG_END ";
	let pattern = "TAG_1[\\s]+id=[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*TAG_END";

	let regex = Regex::new(pattern, EnumSet::empty());
	let mm = regex.exec(subject, 5);

	let expected = ~[subject.slice(9, subject.len()-1), "3", "320", "1", "-321", "11"];
	for i in range::<uint>(0, 5) {
		let substring = match mm.get_substring(i) { Some(s) => s, None => fail!() };
		assert_eq!(expected[i].into_owned(), substring.into_owned());
	}
}

#[test]
fn test_simple_match_all_substring() {
	use pcre::Regex;
	use collections::enum_set::EnumSet;

	let subject = "00:06:08 TAG_1      id=   3,   320,     1,  -321,    11, TAG_END ";
	let pattern = "TAG_1[\\s]+id=[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*TAG_END";

	let regex = Regex::new(pattern, EnumSet::empty());
	let mm = regex.exec(subject, 5);
	//println!("Match: {:?}", mm);
	let substrings = mm.get_all_substring();

	assert_eq!(6, substrings.len());
	let expected = ~[subject.slice(9, subject.len()-1), "3", "320", "1", "-321", "11"];
	for i in range::<uint>(0, 5) {
		assert_eq!(expected[i].into_owned(), substrings.get(i).clone().into_owned());
	}
}

#[test]
fn test_simple_match_all_substring_from() {
	use pcre::Regex;
	use collections::enum_set::EnumSet;

	let subject = "00:06:08 TAG_1      id=   3,   320,     1,  -321,    11, TAG_END ";
	let pattern = "TAG_1[\\s]+id=[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*TAG_END";

	let regex = Regex::new(pattern, EnumSet::empty());
	let mm = regex.exec(subject, 5);
	//println!("Match: {:?}", mm);
	let substrings = mm.get_all_substring_from(1);

	assert_eq!(5, substrings.len());
	let expected = ~["3", "320", "1", "-321", "11"];
	for i in range::<uint>(0, 5) {
		assert_eq!(expected[i].into_owned(), substrings.get(i).clone().into_owned());
	}
}

// tests for low-level (raw) API
// =============================

// test compilation of regex
#[test]
fn test_raw_compile() {
	use pcre::raw::{compile, PCRE_NONE};

	let pcre_comp = compile("Hello", PCRE_NONE);
	assert!(pcre_comp != None);
}

// test study of regex
#[test]
fn test_raw_study() {
	use pcre::raw::{compile, study, PCRE_NONE, PCRE_STUDY_NONE};

	let pcre_extra = match compile("Hello", PCRE_NONE) {
		Some(pc)	=> study(pc, PCRE_STUDY_NONE),
		None		=> None
	};
	assert!(pcre_extra != None);
}

#[test]
fn test_raw_study_jit() {
	use pcre::raw::{compile, study, PCRE_NONE, PCRE_STUDY_JIT_COMPILE};

	let pcre_extra = match compile("Hello", PCRE_NONE) {
		Some(pc)	=> study(pc, PCRE_STUDY_JIT_COMPILE),
		None		=> None
	};
	assert!(pcre_extra != None);
}

// test some basic matching cases
#[test]
fn test_raw_match1 () {
	use pcre::raw::{compile, exec, get_substring, PCRE_NONE, Match};

	let regex = match compile("cat", PCRE_NONE) { Some(r) => r, None => fail!() };
	let subject = "dog and cat";
	let mm = exec(regex, std::ptr::null(), subject, 0, PCRE_NONE, 1);
	let substring = match get_substring(subject, &mm, 0) {
		Some(s)	=> s,
		None	=> { assert!(false); return }
	};
	assert!(match mm { Match(_,_) => true, _ => false });
	assert_eq!(~"cat", substring);
}

#[test]
fn test_raw_match2 () {
	use pcre::raw::{compile, exec, get_substring, PCRE_NONE, Match};

	let regex = match compile("cat", PCRE_NONE) { Some(r) => r, None => fail!() };
	let subject = "catch";
	let mm = exec(regex, std::ptr::null(), subject, 0, PCRE_NONE, 1);
	let substring = match get_substring(subject, &mm, 0) {
		Some(s)	=> s,
		None	=> { assert!(false); return }
	};
	assert!(match mm { Match(_,_) => true, _ => false });
	assert_eq!(~"cat", substring);
}

#[test]
fn test_raw_match_groups() {
	use pcre::raw::{compile, exec, get_substring, PCRE_NONE};

	let subject = "00:06:08 TAG_1      id=   3,   320,     1,  -321,    11, TAG_END ";
	let pattern = "TAG_1[\\s]+id=[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*TAG_END";
	let regex = match compile(pattern, PCRE_NONE) { Some(r) => r, None => fail!() };
	let mm = exec(regex, std::ptr::null(), subject, 0, PCRE_NONE, 5);
	
	// check substrings
	let substrings = ~[subject.slice(9, subject.len()-1), "3", "320", "1", "-321", "11"];
	for i in range::<uint>(0, 5) {
		match get_substring(subject, &mm, i ) {
			Some(s)	=> assert_eq!(substrings[i].into_owned(), s),
			None	=> fail!()
		}
	}
}

#[test]
fn test_raw_match_groups_out_of_bound() {
	use pcre::raw::{compile, exec, get_substring, PCRE_NONE};

	let subject = "00:06:08 TAG_1      id=   3,   320,     1,  -321,    11, TAG_END ";
	let pattern = "TAG_1[\\s]+id=[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*TAG_END";
	let regex = match compile(pattern, PCRE_NONE) { Some(r) => r, None => fail!() };
	let mm = exec(regex, std::ptr::null(), subject, 0, PCRE_NONE, 5);
	
	match get_substring(subject, &mm, 7) {
		Some(s)	=> assert_eq!(~"", s),
		None	=> fail!()
	}
}

#[test]
fn test_raw_unmatch () {
	use pcre::raw::{compile, exec, PCRE_NONE, NoMatch};

	let regex = match compile("cat", PCRE_NONE) { Some(r) => r, None => fail!() };
	let subject = "dog and bird";
	let mm = exec(regex, std::ptr::null(), subject, 0, PCRE_NONE, 1);

	assert!(match mm { NoMatch => true, _ => false });
}

// test JIT based regex matching
#[test]
fn test_raw_jit_match1 () {
	use pcre::raw::{compile, study, exec, get_substring, PCRE_NONE, PCRE_STUDY_JIT_COMPILE, Match};

	let regex = match compile("cat", PCRE_NONE) { Some(r) => r, None => fail!() };
	let extra = match study(regex, PCRE_STUDY_JIT_COMPILE) { Some(e) => e, None => fail!() };
	let subject = "dog and cat";
	let mm = exec(regex, extra, subject, 0, PCRE_NONE, 1);
	let substring = match get_substring(subject, &mm, 0) {
		Some(s)	=> s,
		None	=> { assert!(false); return }
	};
	assert!(match mm { Match(_,_) => true, _ => false });
	assert_eq!(~"cat", substring);
}

#[test]
fn test_raw_jit_match2 () {
	use pcre::raw::{compile, study, exec, get_substring, PCRE_NONE, PCRE_STUDY_JIT_COMPILE, Match};

	let regex = match compile("cat", PCRE_NONE) { Some(r) => r, None => fail!() };
	let extra = match study(regex, PCRE_STUDY_JIT_COMPILE) { Some(e) => e, None => fail!() };
	let subject = "catch";
	let mm = exec(regex, extra, subject, 0, PCRE_NONE, 1);
	let substring = match get_substring(subject, &mm, 0) {
		Some(s)	=> s,
		None	=> fail!()
	};
	assert!(match mm { Match(_,_) => true, _ => false });
	assert_eq!(~"cat", substring);
}

#[test]
fn test_raw_jit_match_groups() {
	use pcre::raw::{compile, study, exec, get_substring, PCRE_NONE, PCRE_STUDY_JIT_COMPILE};

	let subject = "00:06:08 TAG_1      id=   3,   320,     1,  -321,    11, TAG_END ";
	let pattern = "TAG_1[\\s]+id=[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*([-\\d]+),[\\s]*TAG_END";
	let regex = match compile(pattern, PCRE_NONE) { Some(r) => r, None => fail!() };
	let extra = match study(regex, PCRE_STUDY_JIT_COMPILE) { Some(e) => e, None => fail!() };
	let mm = exec(regex, extra, subject, 0, pcre::raw::PCRE_NONE, 5);
	
	// check substrings
	let substrings = ~[subject.slice(9, subject.len()-1), "3", "320", "1", "-321", "11"];
	for i in range::<uint>(0, 5) {
		match get_substring(subject, &mm, i) {
			Some(s)	=> assert_eq!(substrings[i].into_owned(), s),
			None	=> fail!()
		}
	}
}

#[test]
fn test_raw_jit_unmatch () {
	use pcre::raw::{compile, study, exec, PCRE_NONE, PCRE_STUDY_JIT_COMPILE, NoMatch};

	let regex = match compile("cat", PCRE_NONE) { Some(r) => r, None => fail!() };
	let extra = match study(regex, PCRE_STUDY_JIT_COMPILE) { Some(e) => e, None => fail!() };
	let subject = "dog and bird";
	let mm = exec(regex, extra, subject, 0, PCRE_NONE, 1);

	assert!(match mm { NoMatch => true, _ => false });
}


