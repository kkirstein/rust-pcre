// work.rs
// compile & test pcre bindings
//
// vim: ft=rust sw=4 ts=4

#![crate_id = "work"]
#![crate_type = "bin"]

// reference external modules
extern crate time;
extern crate pcre;

// import names
use time::precise_time_ns;

// Main
// ====
fn main() {
	println!("Play with pcre bindings:");
	println!("Using pcre version: {:s}", ::pcre::raw::get_version());

	let pattern = "Hello";
	println!("Compiling an easy pattern ('{:s}'):", pattern);
	let pcre_comp = ::pcre::raw::compile(pattern, pcre::raw::PCRE_NONE);
	println!("{:?}", pcre_comp);
	println!("Studying that pattern:");
	let pcre_extra = match pcre_comp {
		Some(pp)	=> ::pcre::raw::study(pp, ::pcre::raw::PCRE_STUDY_NONE),
		None		=> None
	};
	println!("{:?}", pcre_extra);
	println!("");

	let subject = "Hello World!";
	println!("Matching against '{:s}':", subject);
	let tic1 = precise_time_ns();
	let res1 = match (pcre_comp, pcre_extra) {
		(Some(pc), Some(pe))	=> ::pcre::raw::exec(pc, pe, subject, 0, ::pcre::raw::PCRE_NONE, 1),
		_						=> ::pcre::raw::Error(-100)
	};
	let toc1 = precise_time_ns();
	let match_string = ::pcre::raw::get_substring(subject, &res1, 0);
	println!("Match result: {:?}. Elapsed time {:u}us", match_string, (toc1-tic1)/1000);
	println!("");
	
	// do the same again with JIT enabled
	let pcre_jit = match pcre_comp {
		Some(pc)	=> ::pcre::raw::study(pc, ::pcre::raw::PCRE_STUDY_JIT_COMPILE),
		None		=> None
	};
	println!("Matching JIT-compiled regex against '{:s}':", subject);
	let tic2 = precise_time_ns();
	let res2 = match (pcre_comp, pcre_jit) {
		(Some(pc), Some(pe))	=> ::pcre::raw::exec(pc, pe, subject, 0, ::pcre::raw::PCRE_NONE, 1),
		_						=> ::pcre::raw::Error(-100)
	};
	let toc2 = precise_time_ns();
	let match_string = ::pcre::raw::get_substring(subject, &res2, 0);
	println!("Match result: {:?}. Elapsed time {:u}us", match_string, (toc2-tic2)/1000);
	println!("");

	// free resources
	//match pcre_comp { Some(pc) => ::pcre::free_compiled(pc), None => {} };
	//match pcre_extra { Some(pe) => ::pcre::free_extra(pe), None => {} };
	//match pcre_jit { Some(pj) => ::pcre::free_extra(pj), None => {} };

}


