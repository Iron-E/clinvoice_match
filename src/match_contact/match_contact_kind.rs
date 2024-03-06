mod from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::MatchStr;
use crate::MatchLocation;

/// A [`ContactKind`](winvoice_schema::ContactKind) with [matchable](winvoice_match) fields.
///
/// [`MatchContact`] matches IFF its variant matches.
///
/// # Examples
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # #[cfg(feature = "serde")] {
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::{MatchContactKind, MatchLocation, MatchStr};
/// # {
/// #   let expected = MatchContactKind::Address(MatchLocation {
/// #     name: MatchStr::Contains("New".into()),
/// #     ..Default::default()
/// #   });
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchContactKind>(r#"
/// {
///   "address": {
///     "name": {"contains": "New"}
///   }
/// }
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchContactKind>(r#"
/// address:
///   name:
///     contains: "New"
/// #   "#).unwrap());
/// # }
///
/// // ------------------------------
///
/// # {
/// #   let expected = MatchContactKind::Any;
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchContactKind>(r#"
/// "any"
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchContactKind>("
/// any
/// #   ").unwrap());
/// # }
///
/// // ------------------------------
///
/// # {
/// #   let expected = MatchContactKind::Email("foo@bar.io".to_owned().into());
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchContactKind>(r#"
/// {"email": "foo@bar.io"}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchContactKind>(r#"
/// email: "foo@bar.io"
/// #   "#).unwrap());
/// # }
///
/// // ------------------------------
///
/// # {
/// #   let expected = MatchContactKind::Phone("1-800-555-5555".to_owned().into());
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchContactKind>(r#"
/// {"phone": "1-800-555-5555"}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchContactKind>(r#"
/// phone: "1-800-555-5555"
/// #   "#).unwrap());
/// # }
///
/// // ------------------------------
///
/// # {
/// #   let expected = MatchContactKind::Other("@MyUsername".to_owned().into());
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchContactKind>(r#"
/// {"other": "@MyUsername"}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchContactKind>(r#"
/// other: "@MyUsername"
/// #   "#).unwrap());
/// # }
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "snake_case"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum MatchContactKind
{
	/// Same as [`ContactKind::Address`](winvoice_schema::ContactKind::Address).
	Address(#[cfg_attr(feature = "serde", serde(default))] MatchLocation),

	/// Always match.
	#[default]
	Any,

	/// Same as [`ContactKind::Email`](winvoice_schema::ContactKind::Email).
	Email(#[cfg_attr(feature = "serde", serde(default))] MatchStr<String>),

	/// Same as [`ContactKind::Other`](winvoice_schema::ContactKind::Other).
	Other(#[cfg_attr(feature = "serde", serde(default))] MatchStr<String>),

	/// Same as [`ContactKind::Phone`](winvoice_schema::ContactKind::Phone).
	Phone(#[cfg_attr(feature = "serde", serde(default))] MatchStr<String>),
}
