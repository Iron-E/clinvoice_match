mod default;
mod from;

use core::{cmp::Eq, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A value which wraps another [match](super) type to give it [`Option`] semantics.
///
/// # Examples
///
/// This is an example for how a [`MatchOption`] should be interpreted:
///
/// ```rust
/// use winvoice_match::{Match, MatchOption};
///
/// fn matches(condition: MatchOption<Match<isize>>, opt_x: Option<isize>) -> bool {
///   match condition {
///     MatchOption::Any => true,
///     MatchOption::None => opt_x.is_none(),
///     MatchOption::Some(Match::EqualTo(y)) => opt_x.map_or(false, |x| x == y),
///     MatchOption::Some(_) => unreachable!("Not part of this demonstration"),
///   }
/// }
///
/// assert!(matches(MatchOption::Any, None));
/// assert!(matches(MatchOption::Any, Some(1)));
/// assert!(matches(MatchOption::Some(Match::EqualTo(3)), Some(3)));
/// assert!(matches(MatchOption::None, None));
/// ```
///
/// ## JSON/YAML
///
/// Requires the `serde` feature. Example is for a [`MatchOption`] of [`Match`](super::Match) for
/// [`isize`].
///
/// ```rust
/// # use winvoice_match::{Match, MatchOption};
/// # type M = MatchOption<Match<isize>>;
///
/// # {
/// #   let expected = MatchOption::Any;
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// "any"
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// any
/// #   ").unwrap());
/// # }
///
/// // ----------------------------
///
/// # {
/// #   let expected = MatchOption::Some(3.into());
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"matching": 3}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// matching: 3
/// #   ").unwrap());
/// # }
///
/// // ----------------------------
///
/// # {
/// #   let expected = MatchOption::None;
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// "none"
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// none
/// #   ").unwrap());
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "snake_case"))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchOption<T>
{
	/// Always match.
	Any,

	/// Match IFF some value `v` is null.
	None,

	/// Match IFF some value `v` is present and also matches.
	#[cfg_attr(feature = "serde", serde(rename = "matching"))]
	Some(T),
}

impl<T> MatchOption<T>
{
	/// Transform some [`MatchOption`] of type `T` into another type `U` by providing a mapping
	/// `f`unction.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	///
	/// # Examples
	///
	/// ```rust
	/// use winvoice_match::{Match, MatchOption};
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   MatchOption::Some(Match::EqualTo("5")).map(|m|
	///     m.map(|s| s.parse::<isize>().unwrap())
	///   ),
	///   MatchOption::Some(Match::EqualTo(5))
	/// );
	/// ```
	pub fn map<F, MapTo>(self, f: F) -> MatchOption<MapTo>
	where
		F: FnOnce(T) -> MapTo,
	{
		match self
		{
			Self::Any => MatchOption::Any,
			Self::None => MatchOption::None,
			Self::Some(t) => MatchOption::Some(f(t)),
		}
	}

	/// Transform some [`MatchOption`] of type `T` into another type `U` by providing a mapping
	/// `f`unction.
	///
	/// # See also
	///
	/// * [`MatchOption::map`]
	pub fn map_ref<F, MapTo>(&self, f: F) -> MatchOption<MapTo>
	where
		F: FnOnce(&T) -> MapTo,
	{
		match self
		{
			Self::Any => MatchOption::Any,
			Self::None => MatchOption::None,
			Self::Some(t) => MatchOption::Some(f(t)),
		}
	}
}
