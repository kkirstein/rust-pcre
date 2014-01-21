// pcre.rs
//
// vim: ft=rust sw=4 ts=4
//
// Rust wrapper for pcre regular expressions library
// see docu at http://www.pcre.org/ 
//
// Author: Kay-Uwe Kirstein
//

// type definitions for pcre interface
// ===================================
pub struct Pcre {
	// TODO: insert pcre struct members
}

// low-level (unsafe) functions
// ============================
#[link(name = "pcre/pcre")]
extern {

	fn pcre_compile(pattern: *u8, options: int, error_str: **u8, error_offset: *int, tables: *u8) -> *Pcre; // filterLog
	fn pcre_compile2() -> ();
	fn pcre_config() -> ();
	fn pcre_copy_named_substring() -> ();
	fn pcre_copy_substring() -> ();
	fn pcre_dfa_exec() -> ();
	fn pcre_exec() -> (); // filterLog
	fn pcre_jit_exec() -> ();
	fn pcre_free_substring() -> ();
	fn pcre_free_substring_list() -> ();
	fn pcre_fullinfo() -> ();
	fn pcre_get_named_substring() -> ();
	fn pcre_get_stringnumber() -> ();
	fn pcre_get_substring() -> ();
	fn pcre_get_substring_list() -> (); // filterLog
	fn pcre_refcount() -> ();
	fn pcre_study() -> (); // filterLog
	fn pcre_free_study() -> ();
	fn pcre_version() -> ();

	// Utility functions for byte order swaps
	fn pcre_pattern_to_host_byte_order() -> ();

	// JIT compiler related functions
	fn pcre_jit_stack_alloc() -> ();
	fn pcre_jit_stack_free() -> ();
	fn pcre_assign_jit_stack() -> ();

}

