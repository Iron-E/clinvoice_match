mod exchange;
mod from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::{
	chrono::{DateTime, Utc},
	Id,
};

use super::{Match, MatchEmployee, MatchExpense, MatchJob, MatchOption, MatchSet, MatchStr};

/// A [`Timesheet`](winvoice_schema::Timesheet) with [matchable](winvoice_match) fields.
///
/// [`MatchTimesheet`] matches IFF all of its fields also match.
///
/// # Examples
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// See the documentation for the type of each top-level field (e.g. `id`, `employee`) for
/// information about the types of matching operations which each field supports.
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::{Match, MatchEmployee, MatchExpense, MatchJob, MatchOrganization, MatchStr, MatchTimesheet};
/// # use winvoice_schema::chrono::NaiveDate;
/// # let expected = MatchTimesheet {
/// #   employee: MatchEmployee {
/// #     name: MatchStr::Regex("^[JR]on$".into()),
/// #     ..Default::default()
/// #   },
/// #   expenses: MatchExpense {
/// #     category: "Travel".to_owned().into(),
/// #     ..Default::default()
/// #   }.into(),
/// #   job: MatchJob {
/// #     client: MatchOrganization {
/// #       name: MatchStr::Contains("International".into()),
/// #       ..Default::default()
/// #     },
/// #     ..Default::default()
/// #   },
/// #   time_begin: Match::LessThan(
/// #     NaiveDate::from_ymd_opt(2022, 1, 1).and_then(|d| d.and_hms_opt(0, 0, 0)).unwrap().and_utc(),
/// #   ),
/// #   time_end: None.into(),
/// #   ..Default::default()
/// # };
/// // JSON
/// # assert_eq!(expected, serde_json::from_str::<MatchTimesheet>(r#"
/// {
///   "id": "any",
///   "employee": {
///     "name": {"regex": "^[JR]on$"}
///   },
///   "expenses": {"contains": {
///     "category": "Travel"
///   }},
///   "job": {
///     "client": {
///       "name": {"contains": "International"}
///     }
///   },
///   "time_begin": {"less_than": "2022-01-01T00:00:00Z"},
///   "time_end": "none",
///   "work_notes": "any"
/// }
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchTimesheet>(r#"
/// id: any
/// employee:
///   name:
///     regex: '^[JR]on$'
/// expenses:
///   contains:
///     category: "Travel"
/// job:
///   client:
///     name:
///       contains: "International"
/// time_begin:
///   less_than: "2022-01-01T00:00:00Z"
/// time_end: none
/// work_notes: any
/// # "#).unwrap());
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
	pub time_begin: Match<DateTime<Utc>>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub time_end: MatchOption<Match<DateTime<Utc>>>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub work_notes: MatchStr<String>,
}
