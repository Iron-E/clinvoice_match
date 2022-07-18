use clinvoice_schema::Id;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// A [`Employee`](clinvoice_schema::Employee) with [matchable](clinvoice_match) fields.
///
/// [`MatchEmployee`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchEmployee>(r#"
/// id: any
/// name:
///   regex: 'son\b'
/// status:
///   equal_to: "Hired"
/// title:
///   contains: "C"
/// # "#).is_ok());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchEmployee
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub id: Match<Id>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub name: MatchStr<String>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub status: MatchStr<String>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub title: MatchStr<String>,
}

impl MatchEmployee
{
	/// Return a [`MatchEmployee`] which matches any [`Employee`] whose `id` matches the
	/// `match_condition`.
	pub fn id(match_condition: Match<Id>) -> Self
	{
		Self {
			id: match_condition,
			..Default::default()
		}
	}
}
