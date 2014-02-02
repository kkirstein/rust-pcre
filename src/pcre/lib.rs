// pcre.rs
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

#[allow(dead_code)];

use std::libc::{c_void, c_char, c_int};
use std::str::raw::from_c_str;
use std::ptr::is_not_null;
use std::vec;

// type definitions for pcre interface
// ===================================
pub struct PcreCompiled {
	magic_number: u32,
	size: u32,              /* Total that was malloced */
	options: u32,           /* Public options */
	flags: u32,             /* Private flags */
	limit_match: u32,       /* Limit set from regex */
	limit_recursion: u32,   /* Limit set from regex */
	first_char: u16,        /* Starting character */
	req_char: u16,          /* This character must be seen */
	max_lookbehind: u16,    /* Longest lookbehind (characters) */
	top_bracket: u16,       /* Highest numbered group */
	top_backref: u16,       /* Highest numbered back reference */
	name_table_offset: u16, /* Offset to name table that follows */
	name_entry_size: u16,   /* Size of any name items */
	name_count: u16,        /* Number of name items */
	ref_count: u16,         /* Reference count */
	dummy1: u16,            /* To ensure size is a multiple of 8 */
	dummy2: u16,            /* To ensure size is a multiple of 8 */
	dummy3: u16,            /* To ensure size is a multiple of 8 */
	tables: *u8,            /* Pointer to tables or NULL for std */
	nullpad: *c_void   	    /* NULL padding */
}

pub struct PcreExtra {
	flags: u32,        			/* Bits for which fields are set */
			   study_data: *c_void,    	/* Opaque data from pcre_study() */
			   match_limit: u32,  			/* Maximum number of calls to match() */
			   callout_data: *c_void,  	/* Data passed back in callouts */
			   tables: *u8,    			/* Pointer to character tables */
			   match_limit_recursion: u32,	/* Max recursive calls to match() */
			   mark: **u8,           		/* For passing back a mark pointer */
			   executable_jit: *c_void     /* Contains a pointer to a compiled jit code */
}

pub enum PcreOption {
	PCRE_NONE				= 0x00000000,
	PCRE_CASELESS           = 0x00000001,  /* C1       */
	PCRE_MULTILINE          = 0x00000002,  /* C1       */
	PCRE_DOTALL             = 0x00000004,  /* C1       */
	PCRE_EXTENDED           = 0x00000008,  /* C1       */
	PCRE_ANCHORED           = 0x00000010,  /* C4 E D   */
	PCRE_DOLLAR_ENDONLY     = 0x00000020,  /* C2       */
	PCRE_EXTRA              = 0x00000040,  /* C1       */
	PCRE_NOTBOL             = 0x00000080,  /*    E D J */
	PCRE_NOTEOL             = 0x00000100,  /*    E D J */
	PCRE_UNGREEDY           = 0x00000200,  /* C1       */
	PCRE_NOTEMPTY           = 0x00000400,  /*    E D J */
	PCRE_UTF8               = 0x00000800,  /* C4        )          */
	//PCRE_UTF16              = 0x00000800,  /* C4        ) Synonyms */
	//PCRE_UTF32              = 0x00000800,  /* C4        )          */
	PCRE_NO_AUTO_CAPTURE    = 0x00001000,  /* C1       */
	PCRE_NO_UTF8_CHECK      = 0x00002000,  /* C1 E D J  )          */
	//PCRE_NO_UTF16_CHECK     = 0x00002000,  /* C1 E D J  ) Synonyms */
	//PCRE_NO_UTF32_CHECK     = 0x00002000,  /* C1 E D J  )          */
	PCRE_AUTO_CALLOUT       = 0x00004000,  /* C1       */
	//PCRE_PARTIAL_SOFT       = 0x00008000,  /*    E D J  ) Synonyms */
	PCRE_PARTIAL            = 0x00008000,  /*    E D J  )          */
	
	/* This pair use the same bit. */
	PCRE_NEVER_UTF          = 0x00010000,  /* C1        ) Overlaid */
	//PCRE_DFA_SHORTEST       = 0x00010000,  /*      D    ) Overlaid */
	
	PCRE_DFA_RESTART        = 0x00020000,  /*      D   */
	PCRE_FIRSTLINE          = 0x00040000,  /* C3       */
	PCRE_DUPNAMES           = 0x00080000,  /* C1       */
	PCRE_NEWLINE_CR         = 0x00100000,  /* C3 E D   */
	PCRE_NEWLINE_LF         = 0x00200000,  /* C3 E D   */
	PCRE_NEWLINE_CRLF       = 0x00300000,  /* C3 E D   */
	PCRE_NEWLINE_ANY        = 0x00400000,  /* C3 E D   */
	PCRE_NEWLINE_ANYCRLF    = 0x00500000,  /* C3 E D   */
	PCRE_BSR_ANYCRLF        = 0x00800000,  /* C3 E D   */
	PCRE_BSR_UNICODE        = 0x01000000,  /* C3 E D   */
	PCRE_JAVASCRIPT_COMPAT  = 0x02000000,  /* C5       */
	PCRE_NO_START_OPTIMIZE  = 0x04000000,  /* C2 E D    ) Synonyms */
	//PCRE_NO_START_OPTIMISE  = 0x04000000,  /* C2 E D    )          */
	PCRE_PARTIAL_HARD       = 0x08000000,  /*    E D J */
	PCRE_NOTEMPTY_ATSTART   = 0x10000000,  /*    E D J */
	PCRE_UCP                = 0x20000000  /* C3       */
}

pub enum PcreStudyOption {
	PCRE_STUDY_NONE							= 0x0000,
	PCRE_STUDY_JIT_COMPILE					= 0x0001,
	PCRE_STUDY_JIT_PARTIAL_SOFT_COMPILE		= 0x0002,
	PCRE_STUDY_JIT_PARTIAL_HARD_COMPILE		= 0x0004,
	PCRE_STUDY_EXTRA_NEEDED					= 0x0008
}

pub enum PcreMatch {
	NoMatch,
	MoreMatches(i32, ~[i32]),
	Error(i32),
	Match(i32, ~[i32])
}

// public API
// ==========
pub fn get_version() ->  ~str {
	unsafe {
		from_c_str(pcre_version()) // FIXME: Memory leak?
	}
}

pub fn compile(pattern: &str, options: PcreOption) -> Option<*PcreCompiled> {
	unsafe {
		let error_str = "".to_c_str();
		let mut error_offset = 0 as c_int;

		let pcre_comp = error_str.with_ref( |buf|
											pattern.with_c_str( |pat| pcre_compile(pat, options as c_int, &buf, &mut error_offset, ::std::ptr::null()) ));

		// check for error
		if is_not_null(pcre_comp) {
			Some(pcre_comp)
		} else {
			let error_msg = match error_str.as_str() {
				Some(msg)	=> format!("Error at position {:d}: {:s}\n", error_offset, msg),
				None		=> format!("Unknown error at position {:d}\n", error_offset)
			};
			println(error_msg);
			None
		}
	}
}

pub fn study(pcre_comp: *PcreCompiled, options: PcreStudyOption) -> Option<*PcreExtra> {
	unsafe {
		let error_str = "".to_c_str();
		let pcre_extra = error_str.with_ref( |buf|
											 pcre_study(pcre_comp, options as c_int, &buf) );

		// check for error
		if is_not_null(pcre_extra) {
			Some(pcre_extra)
		} else {
			let error_msg = match error_str.as_str() {
				Some(msg)	=> format!("Error: {:s}\n", msg),
				None		=> ~"Unknown error!\n"
			};
			println(error_msg);
			None
		}
	}
}

pub fn exec(pcre_comp: *PcreCompiled, pcre_extra: *PcreExtra, subject: &str, start_offset: int, options: PcreOption, match_count: uint) -> PcreMatch {
	unsafe {
		let subject_len = subject.len() as c_int;

		// prepare offsets vector
		let offset_count = 3 * (match_count+1);
		let mut offsets: ~[i32] = vec::with_capacity(offset_count);

		// call pcre_exec
		let result = subject.with_c_str( |sub| pcre_exec(pcre_comp, pcre_extra, sub, subject_len, start_offset as c_int, options as c_int, offsets.as_mut_ptr(), offset_count as c_int) );

		// error & result handling
		//offsets.set_len(offset_count);
		//println!("result: {:d}; offset_count: {:u}; offsets: {:?}", result, offset_count, offsets);
		match result {
			-1			=> NoMatch,
			0			=> { offsets.set_len(offset_count); MoreMatches(0, offsets) },
			r if r < -1	=> Error(r),
			r if r > 0	=> { offsets.set_len(3*result as uint); Match(r, offsets) },
			_			=> Error(0)	// this shouldn't happen...
		}
	}
}

pub fn get_substring(subject: &str, match_struct: PcreMatch, match_number: uint) -> Option<&str> {

	let substring = match match_struct {
		Match(count, offsets)		=> {
			let start: uint = offsets[2*count] as uint;
			let end: uint = offsets[2*count+1] as uint;
			subject.slice(start, end)
		},
		MoreMatches(count, offsets)	=> {
			let start: uint = offsets[2*count] as uint;
			let end: uint = offsets[2*count+1] as uint;
			subject.slice(start, end)
		},
		NoMatch						=> "",
		Error(err)					=> ""
	};

	Some(substring)
}

	// call pcre_get_substring
//	unsafe {
//		let buffer = "";
//		//let buffer_raw: *c_char;
//		let result = match match_struct {
//			Match(count, offsets)		=> { buffer.with_c_str( |buf| subject.with_c_str( |sub| pcre_get_substring( sub, offsets.as_ptr(), count, match_number as i32, &buf) ))},
//			//Match(count, offsets)		=> { buffer.with_c_str( |buf| subject.with_c_str( |sub| pcre_get_substring( sub, offsets.as_ptr(), count, match_number as i32, &mut buf) ))},
//			MoreMatches(count, offsets)	=> { buffer.with_c_str( |buf| subject.with_c_str( |sub| pcre_get_substring( sub, offsets.as_ptr(), count/3, match_number as i32, &buf) ))},
//			NoMatch						=> -7,
//			Error(err)					=> err
//		};
//
//		// return as Rust string
//		if (result >= 0) {
//			Some(buffer)
//		} else {
//			None
//		}
//
//		// free allocated memory and return result
//		pcre_free_substring(buffer);
//		substring
//	}
//}

pub fn free_compiled(pcre: *PcreCompiled) -> () {
	unsafe {
		if is_not_null(pcre) {
			pcre_free(pcre);
		}
	}
}

pub fn free_extra(pcre: *PcreExtra) -> () {
	unsafe {
		if is_not_null(pcre) {
			pcre_free_study(pcre);
		}
	}
}

// low-level (unsafe) functions
// ============================
//#[link(name = "pcre")]
#[link(name = "pcre", kind = "static")]
extern {

	fn pcre_compile(pattern: *c_char, options: c_int,
					error_str: **c_char, error_offset: *mut c_int,
					tables: *u8) -> *PcreCompiled; // first prio
	fn pcre_compile2(pattern: *u8, options: c_int,
					 error_code: *c_int, error_str: **u8, error_offset: *c_int,
					 tables: *u8) -> ();
	fn pcre_config(what: c_int, where: *c_void) -> (c_int);
	fn pcre_copy_named_substring(code: *PcreCompiled, subject: *u8, ovector: *c_int,
								 string_count: c_int, string_name: *u8,
								 buffer: *mut u8, size: c_int) -> (c_int);
	fn pcre_copy_substring(code: *PcreCompiled, subject: *u8, ovector: *c_int,
						   string_count: c_int, string_count: c_int,
						   buffer: *mut u8, size: c_int) -> (c_int);
	fn pcre_dfa_exec(pcre: *PcreCompiled, pcre_extra: *PcreExtra, subject: *u8,
					 length: c_int, start_offset: c_int, options: c_int,
					 offsets: *mut c_int, offset_count: c_int,
					 workspace: *mut c_int, ws_count: c_int) -> (c_int);

	/* This function applies a compiled re to a subject string and picks out
	   portions of the string if it matches. Two elements in the vector are set for
	   each substring: the offsets to the start and end of the substring.

	   Arguments:
	   argument_re     points to the compiled expression
	   extra_data      points to extra data or is NULL
	   subject         points to the subject string
	   length          length of subject string (may contain binary zeros)
	   start_offset    where to start in the subject string
	   options         option bits
	   offsets         points to a vector of ints to be filled in with offsets
	   offsetcount     the number of elements in the vector

	   Returns:          > 0 => success; value is the number of elements filled in
	   = 0 => success, but offsets is not big enough
	   -1 => failed to match
	   < -1 => some kind of unexpected problem
	   */
	fn pcre_exec(pcre: *PcreCompiled, pcre_extra: *PcreExtra, subject: *c_char,
				 length: c_int, start_offset: c_int, options: c_int,
				 offsets: *mut c_int, offset_count: c_int) -> (c_int); // first prio
	fn pcre_jit_exec(pcre: *PcreCompiled, pcre_extra: *PcreExtra, subject: *u8,
					 length: c_int, start_offset: c_int, options: c_int,
					 offsets: *mut c_int, offset_count: c_int,
					 stack: *c_void) -> (c_int);
	fn pcre_free_substring(ptr: *c_char) -> ();
	fn pcre_free_substring_list(ptr: **c_char) -> ();
	fn pcre_fullinfo(pcre: *PcreCompiled, pcre_extra: *PcreExtra, what: c_int, where: *c_void) -> (c_int);
	fn pcre_get_named_substring(code: *PcreCompiled, subject: *u8, ovector: *c_int,
								string_count: c_int, string_name: *u8, string_ptr: **u8) -> (c_int);
	fn pcre_get_stringnumber(pcre: *PcreCompiled, string_name: *u8) -> (c_int);
	fn pcre_get_stringable_entries(pcre: *PcreCompiled, string_name: *u8,
								   first_ptr: **u8, last_ptr: **u8) -> (c_int);

	/* This function copies a single captured substring into a piece of new
	store
	
	Arguments:
	  subject        the subject string that was matched
	  ovector        pointer to the offsets table
	  stringcount    the number of substrings that were captured
	                   (i.e. the yield of the pcre_exec call, unless
	                   that was zero, in which case it should be 1/3
	                   of the offset table size)
	  stringnumber   the number of the required substring
	  stringptr      where to put a pointer to the substring
	
	Returns:         if successful:
	                   the length of the string, not including the zero that
	                   is put on the end; can be zero
	                 if not successful:
	                   PCRE_ERROR_NOMEMORY (-6) failed to get store
	                   PCRE_ERROR_NOSUBSTRING (-7) substring not present
	*/
	fn pcre_get_substring(subject: *c_char, ovector: *c_int,
						  string_count: c_int, string_number: c_int, string_ptr: **c_char) -> (c_int);
	fn pcre_get_substring_list(subject: *c_char, ovector: *c_int,
							   string_count: c_int, list_ptr: ***u8) -> (c_int); // first prio
	fn pcre_refcount(pcre: *PcreCompiled, adjust: c_int) -> (c_int);
	fn pcre_study(pcre: *PcreCompiled, options: c_int, error_str: **c_char) -> *PcreExtra; // first prio
	fn pcre_free(pcre_comp: *PcreCompiled) -> ();
	fn pcre_free_study(pcre_extra: *PcreExtra) -> ();
	fn pcre_version() -> *c_char;

	// Utility functions for byte order swaps
	//fn pcre_pattern_to_host_byte_order() -> ();

	// JIT compiler related functions
	//fn pcre_jit_stack_alloc() -> ();
	//fn pcre_jit_stack_free() -> ();
	//fn pcre_assign_jit_stack() -> ();

}


