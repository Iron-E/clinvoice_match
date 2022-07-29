//! `clinvoice_match` contains types that have counterparts with identical layout in
//! [`clinvoice_schema`]. The only difference between the structures in this crate and
//! [`clinvoice_schema`] is that the types in this crate can be used to describe any number of their
//! counterpart types.
//!
//! The ability to "describe" other types comes from [`Match`], [`MatchSet`], and [`MatchStr`].
//! As this is the distinguishing feature of the crate, none of those three types have equivalents
//! in [`clinvoice_schema`].
//!
//! # Features
//!
//! * `serde` adds support for the [`serde`] crate. This crate is intended for and tested
//!   with [`serde_yaml`](https://docs.serde.rs/serde_yaml/) in particular.
//!
//! # Re-exports
//!
//! This crate re-exports [`humantime_serde::Serde`], as it is required to deserialize the
//! `increment` of a [`MatchJob`] via human-readable time (e.g. "15min").
//!
//! # Examples
//!
//! The following [`MatchEmployee`] represents all [`Employee`](clinvoice_schema::Employee)s who
//! meet all of the following criteria:
//!
//! * Have a `name` starting with 'A', 'B', or 'C'.
//! * Have a `status` equal to "Hired".
//! * Have a `title` not equal to "CEO".
//!
//! ```rust
//! use clinvoice_match::{Match, MatchEmployee, MatchStr};
//!
//! let _ = MatchEmployee {
//!   name: MatchStr::Regex("^[ABC]".into()),
//!   status: "Hired".to_string().into(),
//!   title: MatchStr::Not(Box::new("CEO".to_string().into())),
//!   ..Default::default()
//! };
//! ```

#![allow(clippy::drop_non_drop)]
#![forbid(unsafe_code)]
#![warn(
	missing_docs,
	clippy::alloc_instead_of_core,
	clippy::allow_attributes_without_reason,
	clippy::as_underscore,
	clippy::branches_sharing_code,
	clippy::cast_lossless,
	clippy::checked_conversions,
	clippy::cloned_instead_of_copied,
	clippy::dbg_macro,
	clippy::debug_assert_with_mut_call,
	clippy::doc_link_with_quotes,
	clippy::doc_markdown,
	clippy::empty_line_after_outer_attr,
	clippy::empty_structs_with_brackets,
	clippy::enum_glob_use,
	clippy::equatable_if_let,
	clippy::exit,
	clippy::explicit_into_iter_loop,
	clippy::explicit_iter_loop,
	clippy::fallible_impl_from,
	clippy::filetype_is_file,
	clippy::filter_map_next,
	clippy::flat_map_option,
	clippy::fn_to_numeric_cast_any,
	clippy::format_push_string,
	clippy::from_iter_instead_of_collect,
	clippy::get_unwrap,
	clippy::implicit_clone,
	clippy::inefficient_to_string,
	clippy::items_after_statements,
	clippy::manual_assert,
	clippy::manual_ok_or,
	clippy::map_unwrap_or,
	clippy::match_same_arms,
	clippy::missing_const_for_fn,
	clippy::missing_panics_doc,
	clippy::multiple_inherent_impl,
	clippy::mut_mut,
	clippy::needless_continue,
	clippy::option_if_let_else,
	clippy::option_option,
	clippy::range_minus_one,
	clippy::range_plus_one,
	clippy::redundant_closure_for_method_calls,
	clippy::redundant_else,
	clippy::ref_binding_to_reference,
	clippy::ref_option_ref,
	clippy::same_functions_in_if_condition,
	clippy::single_char_lifetime_names,
	clippy::std_instead_of_core,
	clippy::str_to_string,
	clippy::string_add,
	clippy::string_add_assign,
	clippy::string_to_string,
	clippy::try_err,
	clippy::unnecessary_join,
	clippy::unnecessary_wraps,
	clippy::use_self,
	clippy::used_underscore_binding,
	clippy::wildcard_imports
)]

mod r#match;
mod match_contact;
mod match_employee;
mod match_expense;
mod match_invoice;
mod match_job;
mod match_location;
mod match_option;
mod match_organization;
mod match_set;
mod match_str;
mod match_timesheet;

pub use humantime_serde::Serde;
pub use match_contact::{MatchContact, MatchContactKind};
pub use match_employee::MatchEmployee;
pub use match_expense::MatchExpense;
pub use match_invoice::MatchInvoice;
pub use match_job::MatchJob;
pub use match_location::{MatchLocation, MatchOuterLocation};
pub use match_option::MatchOption;
pub use match_organization::MatchOrganization;
pub use match_set::MatchSet;
pub use match_str::MatchStr;
pub use match_timesheet::MatchTimesheet;
pub use r#match::Match;
