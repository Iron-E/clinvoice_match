mod from;

use winvoice_schema::Id;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// A [`Employee`](winvoice_schema::Employee) with [matchable](winvoice_match) fields.
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
/// See the documentation for the type of each top-level field (e.g. `id`, `name`) for information
/// about the types of matching operations which each field supports.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<winvoice_match::MatchEmployee>(r#"
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
