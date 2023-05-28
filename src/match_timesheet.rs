mod exchange;
mod from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::{chrono::NaiveDateTime, Id};

use super::{Match, MatchEmployee, MatchExpense, MatchJob, MatchSet, MatchStr};
use crate::MatchOption;

/// A [`Timesheet`](winvoice_schema::Timesheet) with [matchable](winvoice_match) fields.
///
/// [`MatchTimesheet`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// See the documentation for the type of each top-level field (e.g. `id`, `employee`) for
/// information about the types of matching operations which each field supports.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<winvoice_match::MatchTimesheet>(r#"
/// id: any
/// employee:
///   name:
///     regex: '^[JR]on$'
/// expenses:
///   contains:
///     category:
///       equal_to: "Travel"
/// job:
///   client:
///     name:
///       contains: "Interational"
/// time_begin:
///   less_than: "2022-01-01T00:00:00"
/// time_end: none
/// work_notes: any
/// # "#).is_ok());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MatchTimesheet
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub id: Match<Id>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub employee: MatchEmployee,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub expenses: MatchSet<MatchExpense>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub job: MatchJob,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub time_begin: Match<NaiveDateTime>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub time_end: MatchOption<Match<NaiveDateTime>>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub work_notes: MatchStr<String>,
}
