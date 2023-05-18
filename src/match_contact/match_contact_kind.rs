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
/// ## YAML
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # use winvoice_match::MatchContactKind;
/// # use serde_yaml::from_str;
/// # assert!(from_str::<MatchContactKind>(r#"
/// address:
///   name:
///     contains: "New"
/// # "#).is_ok());
///
/// // ------------------------------
///
/// # assert!(from_str::<MatchContactKind>("
/// any
/// # ").is_ok());
///
/// // ------------------------------
///
/// # assert!(from_str::<MatchContactKind>(r#"
/// email:
///   equal_to: "foo@bar.io"
/// # "#).is_ok());
///
/// // ------------------------------
///
/// # assert!(from_str::<MatchContactKind>(r#"
/// phone:
///   equal_to: "1-800-555-5555"
/// # "#).is_ok());
///
/// // ------------------------------
///
/// # assert!(from_str::<MatchContactKind>(r#"
/// other:
///   equal_to: "@MyUsername"
/// # "#).is_ok());
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
