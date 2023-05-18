mod exchange;
mod from;

use core::time::Duration;

use winvoice_schema::{chrono::NaiveDateTime, Id};
use humantime_serde::Serde;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchInvoice, MatchOrganization, MatchStr};
use crate::MatchOption;

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
/// ## YAML
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// See the documentation for the type of each top-level field (e.g. `client`, `id`) for information
/// about the types of matching operations which each field supports.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<winvoice_match::MatchJob>(r#"
/// client:
///   location:
///     name:
///       contains: "New"
/// date_close: none
/// date_open:
///   in_range: ["2022-05-01T00:00:00", "2022-05-02T00:00:00"]
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
/// # "#).is_ok());
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
	pub date_close: MatchOption<NaiveDateTime>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub date_open: Match<NaiveDateTime>,

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
