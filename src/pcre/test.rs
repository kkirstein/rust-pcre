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
use pcre::{compile, study};

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



