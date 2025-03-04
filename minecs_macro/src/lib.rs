#![forbid( unsafe_code )]
#![warn( clippy::all )]
// pedantic
#![warn(
	clippy::cast_lossless, 
	clippy::cast_possible_truncation, 
	clippy::cast_possible_wrap, 
	clippy::cast_precision_loss, 
	clippy::checked_conversions, 
	clippy::cloned_instead_of_copied, 
	clippy::copy_iterator, 
	clippy::default_trait_access, 
	clippy::enum_glob_use, 
	clippy::explicit_into_iter_loop, 
	clippy::explicit_iter_loop, 
	clippy::float_cmp, 
	clippy::fn_params_excessive_bools, 
	clippy::if_not_else,
	clippy::ignored_unit_patterns,
	clippy::implicit_clone,
	clippy::inconsistent_struct_constructor,
	clippy::index_refutable_slice,
	clippy::inefficient_to_string,
	clippy::invalid_upcast_comparisons, 
	clippy::items_after_statements,
	clippy::iter_filter_is_ok, 
	clippy::iter_filter_is_some, 
	clippy::large_digit_groups, 
	clippy::large_types_passed_by_value,
	clippy::manual_assert,
	clippy::manual_is_variant_and, 
	clippy::manual_let_else,
	clippy::manual_ok_or, 
	clippy::manual_string_new,
	clippy::match_on_vec_items,
	clippy::match_same_arms,
	clippy::match_wild_err_arm,
	clippy::match_wildcard_for_single_variants,
	clippy::mismatching_type_param_order,
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::mut_mut,
	clippy::needless_continue,
	clippy::needless_for_each,
	clippy::needless_pass_by_value,
	clippy::option_as_ref_cloned, 
	clippy::option_option,
	clippy::redundant_closure_for_method_calls,
	clippy::redundant_else,
	clippy::ref_option_ref,
	clippy::return_self_not_must_use,
	clippy::same_functions_in_if_condition,
	clippy::semicolon_if_nothing_returned,
	clippy::should_panic_without_expect,
	clippy::similar_names,
	clippy::single_char_pattern, 
	clippy::single_match_else,
	clippy::stable_sort_primitive,
	clippy::str_split_at_newline,
	clippy::string_add_assign,
	clippy::struct_excessive_bools,
	clippy::struct_field_names,
	clippy::suspicious_xor_used_as_pow, 
	clippy::too_many_lines,
	clippy::trivially_copy_pass_by_ref,
	clippy::unicode_not_nfc,
	clippy::uninlined_format_args,
	clippy::unnecessary_box_returns, 
	clippy::unnecessary_wraps,
	clippy::unnested_or_patterns,
	clippy::unused_self,
)]
// restriction
#![warn(
	//clippy::arithmetic_side_effects, // unlikely to occur, but worth checking
	clippy::else_if_without_else, 
	clippy::empty_enum_variants_with_brackets, 
	clippy::empty_structs_with_brackets, 
	clippy::error_impl_error, 
	clippy::exit, // prefer returnig from main or panic! over `std::process::exit(0)`
	//clippy::field_scoped_visibility_modifiers, // MSRV >= 1.81.0
	//clippy::filetype_is_file, // FS, `is_file` doesn’t cover special file types in unix-like systems, and doesn’t cover symlink in windows
	clippy::float_cmp_const, 
	clippy::fn_to_numeric_cast_any, // low level fn pointer shenanigans
	clippy::format_push_string, // check for known issues with using `write!` as replacement
	clippy::get_unwrap, // prefer proper error handling instead of suggested indexing
	clippy::if_then_some_else_none, 
	clippy::impl_trait_in_params, 
	clippy::indexing_slicing, // may require configuration: suppress-restriction-lint-in-const
	clippy::iter_over_hash_type, 
	clippy::let_underscore_must_use, 
	clippy::min_ident_chars, // conf due to `impl Display`: allowed-idents-below-min-chars = ["..", "f"]
	clippy::missing_inline_in_public_items, 
	clippy::mod_module_files, // style, only relevant in projects with multiple modules
	clippy::module_name_repetitions,
	//clippy::non_zero_suggestions, // MSRV >= 1.83.0
	clippy::panic_in_result_fn, // Functions called from a function returning a Result may invoke a panicking macro. This is not checked.
	clippy::pattern_type_mismatch, 
	clippy::rc_buffer, 
	clippy::rc_mutex, 
	// clippy::renamed_function_params, // MSRV >= 1.80.0
	clippy::rest_pat_in_fully_bound_structs, 
	clippy::same_name_method, 
	clippy::shadow_reuse, 
	clippy::shadow_same, 
	clippy::shadow_unrelated, 
	clippy::string_lit_chars_any, 
	clippy::string_slice, // multi-byte issue, allow with care
	clippy::todo, 
	clippy::undocumented_unsafe_blocks, 
	clippy::unimplemented, 
	clippy::unneeded_field_pattern, 
	clippy::unseparated_literal_suffix, 
	//clippy::unused_trait_names, // MSRV >= 1.83.0
	clippy::unwrap_in_result, 
)]
// style
#![warn(
	clippy::assertions_on_constants,
	clippy::assign_op_pattern,
	clippy::blocks_in_conditions,
	clippy::bool_assert_comparison,
	clippy::collapsible_else_if,
	clippy::collapsible_if,
	clippy::collapsible_match,
	clippy::comparison_chain,
	clippy::comparison_to_empty,
	clippy::enum_variant_names,
	clippy::field_reassign_with_default,
	clippy::get_first,
	clippy::implicit_saturating_add,
	clippy::implicit_saturating_sub,
	clippy::infallible_destructuring_match,
	clippy::inherent_to_string,
	clippy::is_digit_ascii_radix,
	clippy::iter_nth,
	clippy::iter_nth_zero,
	clippy::len_zero,
	clippy::let_and_return,
	clippy::manual_is_ascii_check,
	clippy::manual_map,
	clippy::manual_range_contains,
	clippy::manual_while_let_some,
	clippy::match_overlapping_arm,
	clippy::match_ref_pats,
	clippy::match_result_ok,
	clippy::needless_borrow,
	clippy::needless_range_loop,
	clippy::new_without_default,
	clippy::op_ref,
	clippy::question_mark,
	clippy::redundant_closure,
	clippy::redundant_field_names,
	clippy::redundant_pattern,
	clippy::redundant_pattern_matching,
	clippy::redundant_static_lifetimes,
	clippy::same_item_push,
	clippy::self_named_constructors,
	clippy::should_implement_trait,
	clippy::single_char_add_str,
	clippy::single_match,
	clippy::to_digit_is_some,
	clippy::trim_split_whitespace,
	clippy::unnecessary_fallible_conversions,
	clippy::unnecessary_fold,
	clippy::unnecessary_lazy_evaluations,
	clippy::unnecessary_mut_passed,
	clippy::unnecessary_owned_empty_strings,
	clippy::while_let_on_iterator,
	clippy::write_literal,
	clippy::wrong_self_convention,
)]
#![allow( clippy::tabs_in_doc_comments )]
// nursery
#![warn( clippy::missing_const_for_fn )]
// temporary
#![allow( clippy::result_unit_err )]

//------------------------------------------------------------------------------

use syn::{ parse_macro_input, parse::Parse };
use quote::quote;

//mod v3;
//use v3::*;
mod v4;
use v4::*;

/// Creates ECS structs and implements necessary traits for them.
/// 
/// # Usage
/// 
/// 1. optional derive attribute, ( on top of: `Debug`, `Clone`, `PartialEq` ) fe. `#[derive( serde::Serialize, serde::Deserialize )]`
/// 1. ecs declaration, fe. `ecs MinEcs< CompArray, TestEntity >`
/// 	1. keyword `ecs`
/// 	1. identifier - name of the ecs,
/// 	1. angled braces `<>` surrounding two identifiers separated by a comma: component_array and entity,
/// 1. curly braces `{}` surrounding component declarations ( either or both )
/// 	- keyword `types` followed by square brackets `[]` surrounding comma separated list of not-repeating types; fe. `types [usize, f64]`,
/// 	- comma separated field declarations, such as for struct, in form: identifier, colon, type; fe. `names: Vec< Rc< str >>`.
/// 
/// ```rust
/// # use minecs_common::*;
/// # use minecs_macro::*;
/// # use std::rc::Rc;
/// 
/// minecs!(
/// 	#[derive( /* serde::Serialize, serde::Deserialize, ... */ )]
/// 	ecs MinEcs< CompArray, TestEntity > {
/// 		types [f64, usize]
/// 		some_flag: bool,
/// 		names: Vec< Rc< str >>,
/// 	}
/// );
/// ```
#[proc_macro]
pub fn minecs ( tokens: proc_macro::TokenStream ) -> proc_macro::TokenStream {
	let ir = parse_macro_input!( tokens with CompArray::parse );
	quote! { #ir }.into()
}
