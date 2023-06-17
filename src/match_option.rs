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
/// type M = MatchOption<Match<isize>>;
///
/// fn matches(condition: M, opt_x: Option<isize>) -> bool {
///   match condition {
///     MatchOption::Any => true,
///     MatchOption::None => opt_x.is_none(),
///     MatchOption::NoneOr(Match::EqualTo(y)) => opt_x.map(|x| x == y).unwrap_or(true),
///     MatchOption::Some(Match::EqualTo(y)) => opt_x.map_or(false, |x| x == y),
///     _ => unreachable!("Not part of this demonstration"),
///   }
/// }
///
/// assert!(matches(M::Any, None));
/// assert!(matches(M::Any, Some(1)));
/// assert!(matches(M::None, None));
/// assert!(matches(M::NoneOr(3.into()), None));
/// assert!(matches(M::NoneOr(3.into()), Some(3)));
/// assert!(matches(M::Some(3.into()), Some(3)));
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
/// #   let expected = M::Any;
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
/// #   let expected = M::None;
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
///
/// // ----------------------------
///
/// # {
/// #   let expected = M::NoneOr(3.into());
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"none_or": 3}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// none_or: 3
/// #   ").unwrap());
/// # }
///
/// // ----------------------------
///
/// # {
/// #   let expected = M::Some(3.into());
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"some": 3}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// some: 3
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

	/// Match IFF some value `v` matches either [`None`](MatchOption::None) or
	/// [`Some`](MatchOption::Some).
	NoneOr(T),

	/// Match IFF some value `v` is present and also matches.
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
			Self::NoneOr(t) => MatchOption::NoneOr(f(t)),
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
			Self::NoneOr(t) => MatchOption::NoneOr(f(t)),
			Self::Some(t) => MatchOption::Some(f(t)),
		}
	}
}
