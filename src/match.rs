mod default;
mod exchange;
mod from;

use core::{cmp::Eq, fmt::Debug, mem};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A value which describes the condition which some value of type `T` must meet in order to
/// "_match_".
///
/// # Warnings
///
/// * `Match::Not(Box::new(Match::Any))` is always `false` and often begets a runtime [`Error`](std::error::Error).
/// * You should _never_ use [`Match<Option<T>>`]. Instead, use [`MatchOption<T>`](crate::MatchOption).
///
/// # Examples
///
/// This is an example for how a [`Match`] should be interpreted:
///
/// ```rust
/// use winvoice_match::Match;
///
/// fn matches(condition: Match<isize>, x: isize) -> bool {
///   match condition {
///     Match::And(conditions) => conditions.into_iter().all(|c| matches(c, x)),
///     Match::Any => true,
///     Match::EqualTo(value) => value == x,
///     Match::GreaterThan(value) => x > value,
///     Match::InRange(lower, upper) => lower <= x && x < upper,
///     Match::LessThan(value) => x < value,
///     Match::Not(c) => !matches(*c, x),
///     Match::Or(conditions) => conditions.into_iter().any(|c| matches(c, x)),
///   }
/// }
///
/// assert!(matches(Match::EqualTo(3), 3));
/// assert!(matches(Match::InRange(5, 10), 9));
/// assert!(matches(Match::LessThan(4), 1));
/// assert!(matches(
///   Match::Not(Box::new(Match::Or(vec![
///     Match::GreaterThan(1),
///     Match::LessThan(-1),
///   ]))),
///   0,
/// ));
/// ```
///
/// ## YAML
///
/// Requires the `serde` feature.
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::Match;
/// # type M = Match<isize>;
/// # {
/// #   let expected = Match::And(vec![
/// #     Match::Not(Box::new(3.into())),
/// #     Match::InRange(0, 10),
/// #   ]);
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"and": [
///   {"not": 3},
///   {"in_range": [0, 10]}
/// ]}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// and:
///   - not: 3
///   - in_range: [0, 10]
/// #   ").unwrap());
/// # }
///
/// // -----------------------
///
/// # {
/// #   let expected = Match::Any;
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
/// // -----------------------
///
/// # {
/// #   let expected = Match::EqualTo(3);
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// 3
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// 3
/// #   ").unwrap());
/// # }
///
/// // -----------------------
///
/// # {
/// #   let expected = Match::LessThan(3);
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"less_than": 3}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// less_than: 3
/// #   ").unwrap());
/// # }
///
/// // -----------------------
///
/// # {
/// #   let expected = Match::GreaterThan(3);
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"greater_than": 3}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// greater_than: 3
/// #   ").unwrap());
/// # }
///
/// // -----------------------
///
/// # {
/// #   let expected = Match::InRange(0, 3);
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"in_range": [0, 3]}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// in_range: [0, 3]
/// #   ").unwrap());
/// # }
///
/// // -----------------------
///
/// # {
/// #   let expected = Match::Not(Box::new(3.into()));
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"not": 3}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// not: 3
/// #   ").unwrap());
/// # }
///
/// // -----------------------
///
/// # {
/// #   let expected = Match::Or(vec![Match::GreaterThan(2), 0.into()]);
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"or": [
///   {"greater_than": 2},
///   0
/// ]}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<M>("
/// or:
///   - greater_than: 2
///   - 0
/// #   ").unwrap());
/// # }
/// ```
///
/// ## Warnings
///
/// Never use the following, as it is always `false` and often begets an error:
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::Match;
/// # type M = Match<isize>;
/// # let expected = Match::Not(Match::Any.into());
/// // JSON
/// # assert_eq!(expected, serde_json::from_str::<M>(r#"
/// {"not": "any"}
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<M>("
/// not: any
/// # ").unwrap());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "snake_case"))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Match<T>
{
	/// Match IFF all contained [`Match`]es also match.
	And(Vec<Self>),

	/// Always match.
	Any,

	/// Match IFF some value `v` is greater than  (`>`) this value.
	GreaterThan(T),

	/// Match IFF some value `v` is greater-than-or-equal-to (`>=`) the left-hand contained value,
	/// but is less than (`<`) the right-hand contained value.
	InRange(T, T),

	/// Match IFF some value `v` is less than  (`>`) this value.
	LessThan(T),

	/// Match IFF the contained [`Match`] does _not_ match.
	Not(Box<Self>),

	/// Match IFF any contained [`Match`] matches.
	Or(Vec<Self>),

	/// Match IFF some value `v` matches the contained value.
	#[cfg_attr(feature = "serde", serde(untagged))]
	EqualTo(T),
}

impl<T> Match<T>
{
	/// Combine this condition with some `other` condition using [`Match::And`].
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::Match as M;
	///
	/// let mut cond = M::Any;
	/// cond.and(1.into());
	/// assert_eq!(cond, M::EqualTo(1));
	///
	/// cond.and(2.into());
	/// assert_eq!(cond, M::And(vec![1.into(), 2.into()]));
	///
	/// cond.and(3.into());
	/// assert_eq!(cond, M::And(vec![1.into(), 2.into(), 3.into()]));
	/// ```
	pub fn and(&mut self, other: Self)
	{
		match self
		{
			Self::Any => *self = other,
			Self::And(ref mut vec) => vec.push(other),
			_ =>
			{
				let taken = mem::take(self);
				*self = Self::And(vec![taken, other])
			},
		}
	}

	/// Transform some [`Match`] of type `T` into another type `U` by providing a mapping
	/// `f`unction.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	///
	/// # Examples
	///
	/// ```rust
	/// use winvoice_match::Match;
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   Match::EqualTo("5").map(|s| s.parse::<isize>().unwrap()),
	///   Match::EqualTo(5)
	/// );
	/// ```
	pub fn map<F, U>(self, f: F) -> Match<U>
	where
		F: Copy + Fn(T) -> U,
	{
		match self
		{
			Self::And(match_conditions) => Match::And(match_conditions.into_iter().map(|m| m.map(f)).collect()),
			Self::Any => Match::Any,
			Self::EqualTo(x) => Match::EqualTo(f(x)),
			Self::GreaterThan(x) => Match::GreaterThan(f(x)),
			Self::InRange(low, high) => Match::InRange(f(low), f(high)),
			Self::LessThan(x) => Match::LessThan(f(x)),
			Self::Not(match_condition) => Match::Not(match_condition.map(f).into()),
			Self::Or(match_conditions) => Match::Or(match_conditions.into_iter().map(|m| m.map(f)).collect()),
		}
	}

	/// Transform some [`Match`] of type `T` into another type `U` by providing a mapping
	/// `f`unction.
	///
	/// # See also
	///
	/// * [`Match::map`]
	pub fn map_ref<F, U>(&self, f: F) -> Match<U>
	where
		F: Copy + Fn(&T) -> U,
	{
		match self
		{
			Self::And(match_conditions) => Match::And(match_conditions.iter().map(|m| m.map_ref(f)).collect()),
			Self::Any => Match::Any,
			Self::EqualTo(x) => Match::EqualTo(f(x)),
			Self::GreaterThan(x) => Match::GreaterThan(f(x)),
			Self::InRange(low, high) => Match::InRange(f(low), f(high)),
			Self::LessThan(x) => Match::LessThan(f(x)),
			Self::Not(match_condition) => Match::Not(match_condition.map_ref(f).into()),
			Self::Or(match_conditions) => Match::Or(match_conditions.iter().map(|m| m.map_ref(f)).collect()),
		}
	}

	/// Combine this condition with some `other` condition using [`Match::And`].
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::Match as M;
	///
	/// let mut cond = M::Any;
	/// cond.or(1.into());
	/// assert_eq!(cond, M::EqualTo(1));
	///
	/// cond.or(2.into());
	/// assert_eq!(cond, M::Or(vec![1.into(), 2.into()]));
	///
	/// cond.or(3.into());
	/// assert_eq!(cond, M::Or(vec![1.into(), 2.into(), 3.into()]));
	/// ```
	pub fn or(&mut self, other: Self)
	{
		match self
		{
			Self::Any => *self = other,
			Self::Or(ref mut vec) => vec.push(other),
			_ =>
			{
				let taken = mem::take(self);
				*self = Self::Or(vec![taken, other])
			},
		}
	}
}

impl<T> Match<T>
where
	T: Copy,
{
	/// Transform some [`Match`] of type `T` into another type `U` by providing a mapping
	/// `f`unction.
	///
	/// # See also
	///
	/// * [`Match::map`]
	pub fn map_copied<F, U>(&self, f: F) -> Match<U>
	where
		F: Copy + Fn(T) -> U,
	{
		match self
		{
			Self::And(match_conditions) => Match::And(match_conditions.into_iter().map(|m| m.map_copied(f)).collect()),
			Self::Any => Match::Any,
			Self::EqualTo(x) => Match::EqualTo(f(*x)),
			Self::GreaterThan(x) => Match::GreaterThan(f(*x)),
			Self::InRange(low, high) => Match::InRange(f(*low), f(*high)),
			Self::LessThan(x) => Match::LessThan(f(*x)),
			Self::Not(match_condition) => Match::Not(match_condition.map_copied(f).into()),
			Self::Or(match_conditions) => Match::Or(match_conditions.into_iter().map(|m| m.map_copied(f)).collect()),
		}
	}
}
