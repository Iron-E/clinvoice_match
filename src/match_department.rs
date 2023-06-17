mod from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::Id;

use super::{Match, MatchStr};

/// A [`Department`](winvoice_schema::Department) with [matchable](winvoice_match) fields.
///
/// [`MatchDepartment`] matches IFF all of its fields also match.
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
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::{MatchDepartment, MatchStr};
/// # let expected = MatchDepartment {
/// #   name: MatchStr::Regex(r"son\b".into()),
/// #   ..Default::default()
/// # };
/// // JSON
/// # assert_eq!(expected, serde_json::from_str::<MatchDepartment>(r#"
/// {
///   "id": "any",
///   "name": {"regex": "son\\b"}
/// }
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchDepartment>(r#"
/// id: any
/// name:
///   regex: 'son\b'
/// # "#).unwrap());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchDepartment
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub id: Match<Id>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub name: MatchStr<String>,
}
