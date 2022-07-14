mod exchangeable;

use clinvoice_schema::{chrono::NaiveDateTime, Money};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Match;
use crate::MatchOption;

/// A [`Invoice`](clinvoice_schema::Invoice) with [matchable](clinvoice_match) fields.
///
/// [`MatchInvoice`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchInvoice>(r#"
/// date_issued:
///   in_range: ["2022-01-01T00:00:00", "2023-01-01T00:00:00"]
/// date_paid: none
/// hourly_rate:
///   equal_to:
///     amount: "15.00"
///     currency: USD
/// # "#).is_ok());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchInvoice
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub date_issued: MatchOption<NaiveDateTime>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub date_paid: MatchOption<NaiveDateTime>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub hourly_rate: Match<Money>,
}
