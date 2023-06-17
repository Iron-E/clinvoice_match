mod exchange;
mod from;

use core::time::Duration;

use humantime_serde::Serde;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::{chrono::NaiveDateTime, Id};

use super::{Match, MatchInvoice, MatchOrganization, MatchStr};
use crate::{MatchOption, MatchSet};

/// A [`Job`](winvoice_schema::Job) with [matchable](winvoice_match) fields.
///
/// [`MatchJob`] matches IFF all of its fields also match.
///
/// # Notes
///
/// * See [`humantime_serde`] for the syntax of matched data in the `increment` field.
///
/// # Examples
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// See the documentation for the type of each top-level field (e.g. `client`, `id`) for information
/// about the types of matching operations which each field supports.
///
/// ```rust
/// # use core::time::Duration;
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::{Match, MatchInvoice, MatchJob, MatchLocation, MatchOrganization, MatchSet, MatchStr};
/// # use winvoice_schema::chrono::NaiveDate;
/// # let expected = MatchJob {
/// #   client: MatchOrganization {
/// #     location: MatchLocation {
/// #       name: MatchStr::Contains("New".into()),
/// #       ..Default::default()
/// #     },
/// #     ..Default::default()
/// #   },
/// #   date_close: None.into(),
/// #   date_open: Match::InRange(
/// #     NaiveDate::from_ymd_opt(2022, 5, 1).and_then(|d| d.and_hms_opt(0, 0, 0)).unwrap(),
/// #     NaiveDate::from_ymd_opt(2022, 5, 2).and_then(|d| d.and_hms_opt(0, 0, 0)).unwrap(),
/// #   ),
/// #   departments: MatchSet::Contains("Executive".to_owned().into()),
/// #   increment: Match::EqualTo(Duration::from_secs(60 * 5).into()),
/// #   invoice: MatchInvoice {
/// #     date_paid: None.into(),
/// #     date_issued: None.into(),
/// #     ..Default::default()
/// #   },
/// #   notes: MatchStr::Contains("here is some multiline text.\nand some more text.\n".into()),
/// #   ..Default::default()
/// # };
/// // JSON
/// # assert_eq!(expected, serde_json::from_str::<MatchJob>(r#"
/// {
///   "client": {
///     "location": {
///       "name": {"contains": "New"}
///     }
///   },
///   "date_close": "none",
///   "date_open": {"in_range": ["2022-05-01T00:00:00", "2022-05-02T00:00:00"]},
///   "departments": {"contains": {"equal_to": "Executive"}},
///   "id": "any",
///   "increment": {"equal_to": "5min"},
///   "invoice": {
///     "date_paid": "none",
///     "date_issued": "none"
///   },
///   "notes": {"contains": "here is some multiline text.\nand some more text.\n"},
///   "objectives": "any"
/// }
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchJob>(r#"
/// client:
///   location:
///     name:
///       contains: "New"
/// date_close: none
/// date_open:
///   in_range: ["2022-05-01T00:00:00", "2022-05-02T00:00:00"]
/// departments:
///   contains:
///     equal_to: "Executive"
/// id: any
/// increment:
///   equal_to: "5min"
/// invoice:
///   date_paid: none
///   date_issued: none
/// notes:
///   contains: |
///     here is some multiline text.
///     and some more text.
/// objectives: any
/// # "#).unwrap());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MatchJob
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub client: MatchOrganization,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub date_close: MatchOption<Match<NaiveDateTime>>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub date_open: Match<NaiveDateTime>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub departments: MatchSet<MatchStr<String>>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub id: Match<Id>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub increment: Match<Serde<Duration>>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub invoice: MatchInvoice,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub notes: MatchStr<String>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub objectives: MatchStr<String>,
}
