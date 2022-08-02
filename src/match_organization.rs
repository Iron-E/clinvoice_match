mod from;

use clinvoice_schema::Id;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchLocation, MatchStr};

/// A [`Organization`](clinvoice_schema::Organization) with [matchable](clinvoice_match) fields.
///
/// [`MatchOrganization`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// See the documentation for the type of each top-level field (e.g. `id`, `location`) for
/// information about the types of matching operations which each field supports.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchOrganization>(r#"
/// id: any
/// location:
///   outer:
///     some:
///       name:
///         equal_to: "Mexico"
/// name:
///   equal_to: "Some Company"
/// # "#).is_ok());
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
