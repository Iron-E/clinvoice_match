mod from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::Id;

use super::{Match, MatchLocation, MatchStr};

/// A [`Organization`](winvoice_schema::Organization) with [matchable](winvoice_match) fields.
///
/// [`MatchOrganization`] matches IFF all of its fields also match.
///
/// # Examples
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// See the documentation for the type of each top-level field (e.g. `id`, `location`) for
/// information about the types of matching operations which each field supports.
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::{MatchLocation, MatchOrganization};
/// # let expected = MatchOrganization {
/// #   location: MatchLocation {
/// #     outer: Some(MatchLocation {
/// #       name: "Mexico".to_owned().into(),
/// #       ..Default::default()
/// #     }.into()).into(),
/// #    ..Default::default()
/// #   },
/// #   name: "Some Company".to_owned().into(),
/// #   ..Default::default()
/// # };
/// // JSON
/// # assert_eq!(expected, serde_json::from_str::<MatchOrganization>(r#"
/// {
///   "id": "any",
///   "location": {
///     "outer": {"some": {
///       "name": {"equal_to": "Mexico"}
///     }}
///   },
///   "name": {"equal_to": "Some Company"}
/// }
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchOrganization>(r#"
/// id: any
/// location:
///   outer:
///     some:
///       name:
///         equal_to: "Mexico"
/// name:
///   equal_to: "Some Company"
/// # "#).unwrap());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MatchOrganization
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub id: Match<Id>,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub location: MatchLocation,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub name: MatchStr<String>,
}
