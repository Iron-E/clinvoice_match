mod exchange;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::{chrono::NaiveDateTime, Money};

use super::Match;
use crate::MatchOption;

/// A [`Invoice`](winvoice_schema::Invoice) with [matchable](winvoice_match) fields.
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
/// See the documentation for the type of each top-level field (e.g. `date_issued`, `date_paid`) for
/// information about the types of matching operations which each field supports.
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::{Match, MatchInvoice};
/// # use winvoice_schema::{chrono::NaiveDate, Currency, Money};
/// # let expected = MatchInvoice {
/// #   date_issued: Some(Match::InRange(
/// #     NaiveDate::from_ymd_opt(2022, 1, 1).and_then(|d| d.and_hms_opt(0, 0, 0)).unwrap(),
/// #     NaiveDate::from_ymd_opt(2023, 1, 1).and_then(|d| d.and_hms_opt(0, 0, 0)).unwrap(),
/// #   )).into(),
/// #   date_paid: None.into(),
/// #   hourly_rate: Money::new(15_00, 2, Currency::Usd).into(),
/// #   ..Default::default()
/// # };
/// // JSON
/// # assert_eq!(expected, serde_json::from_str::<MatchInvoice>(r#"
/// {
///   "date_issued": {"matching": {"in_range": ["2022-01-01T00:00:00", "2023-01-01T00:00:00"]}},
///   "date_paid": "none",
///   "hourly_rate": {"amount": "15.00", "currency": "USD"}
/// }
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchInvoice>(r#"
/// date_issued:
///   matching:
///     in_range: ["2022-01-01T00:00:00", "2023-01-01T00:00:00"]
/// date_paid: none
/// hourly_rate:
///   amount: "15.00"
///   currency: USD
/// # "#).unwrap());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchInvoice
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub date_issued: MatchOption<Match<NaiveDateTime>>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub date_paid: MatchOption<Match<NaiveDateTime>>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub hourly_rate: Match<Money>,
}
