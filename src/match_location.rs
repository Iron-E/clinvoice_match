mod from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winvoice_schema::{Currency, Id};

use super::{Match, MatchOption, MatchStr};

/// A [`Location`](winvoice_schema::Location) with [matchable](winvoice_match) fields.
///
/// [`MatchLocation`] matches IFF all of its fields also match.
///
/// # Examples
///
/// Requires the `serde` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// See the documentation for the type of each top-level field (e.g. `id`, `outer`) for information
/// about the types of matching operations which each field supports.
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use winvoice_schema::Currency;
/// # use winvoice_match::{Match, MatchLocation};
/// # let expected = MatchLocation {
/// # outer: Some(MatchLocation {
/// #   currency: Some(Match::from(Currency::Usd)).into(),
/// #   name: "Europe".to_owned().into(),
/// #   ..Default::default()
/// # }.into()).into(),
/// # name: "Sweden".to_owned().into(),
/// # ..Default::default()
/// # };
/// // JSON
/// # assert_eq!(expected, serde_json::from_str::<MatchLocation>(r#"
/// {
///   "id": "any",
///   "outer": {"some": {
///     "currency": {"some": "USD"},
///     "name": "Europe"
///   }},
///   "name": "Sweden"
/// }
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchLocation>(r#"
/// id: any
/// outer:
///   some:
///     currency:
///       some: USD
///     name: Europe
/// name: Sweden
/// # "#).unwrap());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchLocation
{
	#[allow(missing_docs)]
	#[cfg_attr(feature = "serde", serde(default))]
	pub currency: MatchOption<Match<Currency>>,

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
