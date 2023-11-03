//! [`winvoice_match`](crate) contains counterpart types to those in [`winvoice_schema`], the difference being that
//! these types are built upon *descriptors* rather than concrete values. For example, the following [`MatchEmployee`]
//! represents all [`Employee`](winvoice_schema::Employee)s who:
//!
//! * Have a `name` starting with 'A', 'B', or 'C'.
//! * Have a `status` equal to "Hired".
//! * Have a `title` not equal to "CEO".
//!
//! ```rust
//! use winvoice_match::{Match, MatchEmployee, MatchStr};
//!
//! let _ = MatchEmployee {
//!   department: "Executive".to_owned().into(),
//!   name: MatchStr::Regex("^[ABC]".into()),
//!   title: MatchStr::Not(Box::new("CEO".to_owned().into())),
//!   ..Default::default()
//! };
//! ```
//!
//! The ability to "describe" values comes from [`Match`], [`MatchOption`], [`MatchSet`], and [`MatchStr`].
//!
//! # Features
//!
//! * `serde` adds support for the [`serde`] crate. This crate is tested using
//!   [`serde_json`](https://docs.serde.rs/serde_json/) and [`serde_yaml`](https://docs.serde.rs/serde_yaml/).
//!
//! # Re-exports
//!
//! This crate re-exports [`humantime_serde::Serde`], as it is required to deserialize the
//! `increment` of a [`MatchJob`] via human-readable time (e.g. "15min").

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
mod match_department;
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
pub use match_department::MatchDepartment;
pub use match_employee::MatchEmployee;
pub use match_expense::MatchExpense;
pub use match_invoice::MatchInvoice;
pub use match_job::MatchJob;
pub use match_location::MatchLocation;
pub use match_option::MatchOption;
pub use match_organization::MatchOrganization;
pub use match_set::MatchSet;
pub use match_str::MatchStr;
pub use match_timesheet::MatchTimesheet;
pub use r#match::Match;
