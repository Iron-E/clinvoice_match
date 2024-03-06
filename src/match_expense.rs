mod exchange;
mod from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::{Id, Money};

use super::{Match, MatchStr};

/// A [`Expense`](winvoice_schema::Expense) with [matchable](winvoice_match) fields.
///
/// [`MatchExpense`] matches IFF all of its fields also match.
///
/// # Examples
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// See the documentation for the type of each top-level field (e.g. `cost`, `id`) for information
/// about the types of matching operations which each field supports.
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::{Match, MatchExpense, MatchStr};
/// # use winvoice_schema::{Currency, Money};
/// # use uuid::uuid;
/// # let expected = MatchExpense {
/// #  category: MatchStr::Regex(r"^\s*([Ff]ood|[Tt]ravel)\s*$".into()),
/// #  cost: Match::GreaterThan(Money::new(50_00, 2, Currency::Usd)),
/// #  description: MatchStr::Contains("need".into()),
/// #  timesheet_id: uuid!("e1d0b735-2b36-43e9-8d04-967573eed612").into(),
/// #  ..Default::default()
/// # };
/// // JSON
/// # #[cfg(feature = "serde")] {
/// # assert_eq!(expected, serde_json::from_str::<MatchExpense>(r#"
/// {
///   "category": {"regex": "^\\s*([Ff]ood|[Tt]ravel)\\s*$"},
///   "cost": {"greater_than": {"amount": "50.00", "currency": "USD"}},
///   "description": {"contains": "need"},
///   "id": "any",
///   "timesheet_id": "e1d0b735-2b36-43e9-8d04-967573eed612"
/// }
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchExpense>(r#"
/// category:
///   regex: '^\s*([Ff]ood|[Tt]ravel)\s*$'
/// cost:
///   greater_than:
///     amount: "50.00"
///     currency: USD
/// description:
///   contains: "need"
/// id: any
/// timesheet_id: "e1d0b735-2b36-43e9-8d04-967573eed612"
/// # "#).unwrap());
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchExpense
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub category: MatchStr<String>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub cost: Match<Money>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub description: MatchStr<String>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub id: Match<Id>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub timesheet_id: Match<Id>,
}
