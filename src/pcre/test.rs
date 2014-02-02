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

// test compilation of regex
#[test]
fn test_compile() {
	let pcre_comp = ::pcre::compile("Hello", pcre::PCRE_NONE);
	assert!(pcre_comp != None);
}


