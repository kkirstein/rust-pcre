// work.rs
// compile & test pcre bindings
//
// vim: ft=rust sw=4 ts=4

#[crate_id = "work"];
#[crate_type = "bin"];

// reference pcre module
extern mod pcre;
//use pcre::{get_version, compile, study, free_compiled, free_extra};

// Main
// ====
fn main() {
	println("Play with pcre bindings:");
	println!("Using pcre version: {:s}", ::pcre::get_version());

	let pattern = "Hello";
	println!("Compiling an easy pattern ('{:s}'):", pattern);
	let pcre_comp = ::pcre::compile(pattern, pcre::PCRE_NONE);
	println!("{:?}", pcre_comp);
	println("Studying that pattern:");
	let pcre_extra = match pcre_comp {
		Some(pp)	=> ::pcre::study(pp, ::pcre::PCRE_NONE),
		None		=> None
	};
	println!("{:?}", pcre_extra);

	let subject = "Hello World!";
	println!("Matching against '{:s}':", subject);
	let res = match (pcre_comp, pcre_extra) {
		(Some(pc), Some(pe))	=> ::pcre::exec(pc, pe, subject, 0, ::pcre::PCRE_NONE, 1),
		_						=> ::pcre::Error(-100)
	};
	println!("{:?}", res);
	

	// some basic checks
	assert!(pcre_comp != None);
	assert!(pcre_extra != None);


	// free resources
	match (pcre_comp, pcre_extra) {
		(Some(pc), Some(pe))	=> { ::pcre::free_compiled(pc); ::pcre::free_extra(pe); },
		(Some(pc), None)		=> { ::pcre::free_compiled(pc); },
		(None, Some(pe))		=> { ::pcre::free_extra(pe); },
		(None, None)			=> {}
		}
}


