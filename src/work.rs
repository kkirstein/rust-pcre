// work.rs
// compile & test pcre bindings
//
// vim: ft=rust sw=4 ts=4

#[crate_id = "work"];
#[crate_type = "bin"];

// reference external modules
extern mod extra;
extern mod pcre;

// import names
use extra::time::precise_time_ns;

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
		Some(pp)	=> ::pcre::study(pp, ::pcre::PCRE_STUDY_NONE),
		None		=> None
	};
	println!("{:?}", pcre_extra);
	println("");

	let subject = "Hello World!";
	println!("Matching against '{:s}':", subject);
	let tic1 = precise_time_ns();
	let res1 = match (pcre_comp, pcre_extra) {
		(Some(pc), Some(pe))	=> ::pcre::exec(pc, pe, subject, 0, ::pcre::PCRE_NONE, 1),
		_						=> ::pcre::Error(-100)
	};
	let toc1 = precise_time_ns();
	let match_string = ::pcre::get_substring(subject, res1, 0);
	println!("Match result: {:?}. Elapsed time {:u}us", match_string, (toc1-tic1)/1000);
	println("");
	
	// do the same again with JIT enabled
	let pcre_jit = match pcre_comp {
		Some(pc)	=> ::pcre::study(pc, ::pcre::PCRE_STUDY_JIT_COMPILE),
		None		=> None
	};
	println!("Matching JIT-compiled regex against '{:s}':", subject);
	let tic2 = precise_time_ns();
	let res2 = match (pcre_comp, pcre_extra) {
		(Some(pc), Some(pe))	=> ::pcre::exec(pc, pe, subject, 0, ::pcre::PCRE_NONE, 1),
		_						=> ::pcre::Error(-100)
	};
	let toc2 = precise_time_ns();
	println!("Match result: {:?}. Elapsed time {:u}us", res2, (toc2-tic2)/1000);
	println("");

	// free resources
	match (pcre_comp, pcre_extra) {
		(Some(pc), Some(pe))	=> { ::pcre::free_compiled(pc); ::pcre::free_extra(pe); },
		(Some(pc), None)		=> { ::pcre::free_compiled(pc); },
		(None, Some(pe))		=> { ::pcre::free_extra(pe); },
		(None, None)			=> {}
	}
	match pcre_jit { Some(pj) => ::pcre::free_extra(pj), None => {} };

}


