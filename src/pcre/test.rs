// test.rs
//
// vim: ft=rust sw=4 ts=4
//
// Test case for PCRE Rust bindings
// see docu at http://www.pcre.org/ 
//
// Author: Kay-Uwe Kirstein
//

#[license = "BSD"];

extern mod pcre;
use pcre::{compile, study, exec, get_substring};

// test compilation of regex
#[test]
fn test_raw_compile() {
	let pcre_comp = compile("Hello", pcre::PCRE_NONE);
	assert!(pcre_comp != None);
	//match pcre_comp {
	//	Some(pc)	=> {
	//		assert_eq!(pc.magic_number, 0123);
	//		assert_eq!(pc.options, 0);
	//	}
	//	None		=> assert!(false)
	//}
}

// test study of regex
#[test]
fn test_raw_study() {
	let pcre_extra = match compile("Hello", pcre::PCRE_NONE) {
		Some(pc)	=> study(pc, pcre::PCRE_STUDY_NONE),
		None		=> None
	};
	assert!(pcre_extra != None);
}

#[test]
fn test_raw_study_jit() {
	let pcre_extra = match compile("Hello", pcre::PCRE_NONE) {
		Some(pc)	=> study(pc, pcre::PCRE_STUDY_JIT_COMPILE),
		None		=> None
	};
	assert!(pcre_extra != None);
}

// test some basic matching cases
#[test]
fn test_raw_match () {
	let regex = match compile("cat", pcre::PCRE_NONE) { Some(r) => r, None => { assert!(false); return }};
	let subject = "dog and cat";
	let mm = exec(regex, std::ptr::null(), subject, 0, pcre::PCRE_NONE, 1);
	let substring = match get_substring(subject, &mm, 0) {
		Some(s)	=> s,
		None	=> { assert!(false); return }
	};
	assert!(match mm { pcre::Match(_,_) => true, _ => false });
	assert_eq!(~"cat", substring);

	let subject = "catch";
	let mm = exec(regex, std::ptr::null(), subject, 0, pcre::PCRE_NONE, 1);
	let substring = match get_substring(subject, &mm, 0) {
		Some(s)	=> s,
		None	=> { assert!(false); return }
	};
	assert!(match mm { pcre::Match(_,_) => true, _ => false });
	assert_eq!(~"cat", substring);
}

#[test]
fn test_raw_unmatch () {
	let regex = match compile("cat", pcre::PCRE_NONE) { Some(r) => r, None => { assert!(false); return }};
	let subject = "dog and bird";
	let mm = exec(regex, std::ptr::null(), subject, 0, pcre::PCRE_NONE, 1);

	assert!(match mm { pcre::NoMatch => true, _ => false });
}

// test JIT based regex matching
#[test]
fn test_jit_match () {
	let regex = match compile("cat", pcre::PCRE_NONE) { Some(r) => r, None => { assert!(false); return }};
	let extra = match study(regex, pcre::PCRE_STUDY_JIT_COMPILE) { Some(e) => e, None => { assert!(false); return }};
	let subject = "dog and cat";
	let mm = exec(regex, extra, subject, 0, pcre::PCRE_NONE, 1);
	let substring = match get_substring(subject, &mm, 0) {
		Some(s)	=> s,
		None	=> { assert!(false); return }
	};
	assert!(match mm { pcre::Match(_,_) => true, _ => false });
	assert_eq!(~"cat", substring);

	let subject = "catch";
	let mm = exec(regex, extra, subject, 0, pcre::PCRE_NONE, 1);
	let substring = match get_substring(subject, &mm, 0) {
		Some(s)	=> s,
		None	=> { assert!(false); return }
	};
	assert!(match mm { pcre::Match(_,_) => true, _ => false });
	assert_eq!(~"cat", substring);
}

#[test]
fn test_jit_unmatch () {
	let regex = match compile("cat", pcre::PCRE_NONE) { Some(r) => r, None => { assert!(false); return }};
	let extra = match study(regex, pcre::PCRE_STUDY_JIT_COMPILE) { Some(e) => e, None => { assert!(false); return }};
	let subject = "dog and bird";
	let mm = exec(regex, extra, subject, 0, pcre::PCRE_NONE, 1);

	assert!(match mm { pcre::NoMatch => true, _ => false });
}


