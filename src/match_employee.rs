mod from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::Id;

use super::{Match, MatchDepartment, MatchStr};

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
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::{MatchEmployee, MatchStr};
/// # let expected = MatchEmployee {
/// #   active: true.into(),
/// #   department: "Executive".to_owned().into(),
/// #   name: MatchStr::Regex(r"son\b".into()),
/// #   title: MatchStr::Contains("C".into()),
/// #   ..Default::default()
/// # };
/// // JSON
/// # #[cfg(feature = "serde")] {
/// # assert_eq!(expected, serde_json::from_str::<MatchEmployee>(r#"
/// {
///   "active": true,
///   "department": {"name": "Executive"},
///   "id": "any",
///   "name": {"regex": "son\\b"},
///   "title": {"contains": "C"}
/// }
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchEmployee>(r#"
/// active: true
/// department:
///   name: "Executive"
/// id: any
/// name:
///   regex: 'son\b'
/// title:
///   contains: "C"
/// # "#).unwrap());
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchEmployee
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub active: Match<bool>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub department: MatchDepartment,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub id: Match<Id>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub name: MatchStr<String>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub title: MatchStr<String>,
}
