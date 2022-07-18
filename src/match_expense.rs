mod exchangeable;

use clinvoice_schema::{Id, Money};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// A [`Expense`](clinvoice_schema::Expense) with [matchable](clinvoice_match) fields.
///
/// [`MatchExpense`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchExpense>(r#"
/// category:
///   regex: '^\s*([Ff]ood|[Tt]ravel)\s*$'
/// cost:
///   greater_than:
///     amount: "50.00"
///     currency: USD
/// description:
///   contains: "need"
/// id: any
/// timesheet_id:
///   equal_to: 4
/// # "#).is_ok());
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

impl MatchExpense
{
	/// Return a [`MatchExpense`] which matches any [`Expense`] whose `id` matches the
	/// `match_condition`.
	pub fn id(match_condition: Match<Id>) -> Self
	{
		Self {
			id: match_condition,
			..Default::default()
		}
	}
}
