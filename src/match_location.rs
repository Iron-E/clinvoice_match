mod from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::Id;

use super::{Match, MatchStr};
use crate::MatchOption;

/// A [`Location`](winvoice_schema::Location) with [matchable](winvoice_match) fields.
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
/// See the documentation for the type of each top-level field (e.g. `id`, `outer`) for information
/// about the types of matching operations which each field supports.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<winvoice_match::MatchLocation>(r#"
/// id: any
/// outer:
///   name:
///     equal_to: "Europe"
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
	pub outer: MatchOption<Box<MatchLocation>>,
}
