mod match_outer_location;

use clinvoice_schema::Id;
pub use match_outer_location::MatchOuterLocation;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// A [`Location`](clinvoice_schema::Location) with [matchable](clinvoice_match) fields.
///
/// [`MatchLocation`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchLocation>(r#"
/// id: any
/// outer:
///   some:
///     name:
///       equal_to: "Europe"
/// name:
///   equal_to: "Sweden"
/// # "#).is_ok());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchLocation
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub id: Match<Id>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub name: MatchStr<String>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub outer: MatchOuterLocation,
}

impl MatchLocation
{
	/// Return a [`MatchLocation`] which matches any [`Location`] whose `id` is
	/// [`EqualTo`](Match::EqualTo) `i`.
	pub fn id(i: Id) -> Self
	{
		Self {
			id: i.into(),
			..Default::default()
		}
	}
}
