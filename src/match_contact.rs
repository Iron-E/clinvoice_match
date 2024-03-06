mod from;
mod match_contact_kind;

pub use match_contact_kind::MatchContactKind;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::MatchStr;

/// A [`Contact`](winvoice_schema::Contact) with [matchable](winvoice_match) fields.
///
/// [`MatchContact`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// See the documentation for the type of each top-level field (e.g. `category`, `cost`) for
/// information about the types of matching operations which each field supports.
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::{MatchContact, MatchContactKind};
/// # let expected = MatchContact {
/// #   kind: MatchContactKind::Email("foo@bar.io".to_owned().into()),
/// #   label: "Primary Email".to_owned().into(),
/// # };
/// // JSON
/// # #[cfg(feature = "serde")] {
/// # assert_eq!(expected, serde_json::from_str::<MatchContact>(r#"
/// {
///   "kind": {
///     "email": "foo@bar.io"
///   },
///   "label": "Primary Email"
/// }
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchContact>(r#"
/// kind:
///   email: "foo@bar.io"
/// label: "Primary Email"
/// # "#).unwrap());
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchContact
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub kind: MatchContactKind,

	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub label: MatchStr<String>,
}
