mod default;
mod from;

use core::mem;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A value which describes the condition which some string of type `T` must meet in order to
/// "_match_".
///
/// # Warnings
///
/// * `MatchStr::Not(Box::new(MatchStr::Any))` is always `false` and often begets a runtime
///   [`Error`](std::error::Error).
///
/// # Examples
///
/// This is an example for how a [`MatchStr`] should be interpreted:
///
/// ```rust
/// use winvoice_match::MatchStr;
/// use regex::Regex;
///
/// fn matches(condition: MatchStr<&str>, x: &str) -> bool {
///   match condition {
///     MatchStr::And(conditions) => conditions.into_iter().all(|c| matches(c, x)),
///     MatchStr::Any => true,
///     MatchStr::Contains(value) => x.contains(value),
///     MatchStr::EqualTo(value) => value == x,
///     MatchStr::Not(c) => !matches(*c, x),
///     MatchStr::Or(conditions) => conditions.into_iter().any(|c| matches(c, x)),
///     MatchStr::Regex(value) => Regex::new(value).unwrap().is_match(x),
///   }
/// }
///
/// assert!(matches(MatchStr::Contains("f"), "foo"));
/// assert!(matches(MatchStr::EqualTo("foo"), "foo"));
/// assert!(matches(MatchStr::Regex("fo{2,}"), "foo"));
/// assert!(matches(
///   MatchStr::Not(Box::new(MatchStr::Or(vec![
///     MatchStr::Contains("b"),
///     MatchStr::Contains("a")
///   ]))),
///   "foo",
/// ));
/// ```
///
/// ## JSON/YAML
///
/// Requires the `serde` feature.
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use winvoice_match::MatchStr;
/// # type MatchString = MatchStr<String>;
/// # {
/// #   let expected = MatchStr::And(vec![
/// #     MatchStr::Contains("f".into()),
/// #     MatchStr::Regex("o{2,}$".into()),
/// #   ]);
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchString>(r#"
/// {
///   "and": [
///     {"contains": "f"},
///     {"regex": "o{2,}$"}
///   ]
/// }
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchString>(r#"
/// and:
///   - contains: "f"
///   - regex: 'o{2,}$'
/// #   "#).unwrap());
/// # }
///
/// // -------------------
///
/// # {
/// #   let expected = MatchStr::Any;
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchString>(r#"
/// "any"
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchString>("
/// any
/// #   ").unwrap());
/// # }
///
/// // -------------------
///
/// # {
/// #   let expected = MatchStr::Contains("foo".into());
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchString>(r#"
/// {"contains": "foo"}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchString>(r#"
/// contains: "foo"
/// #   "#).unwrap());
/// # }
///
/// // -------------------
///
/// # {
/// #   let expected = MatchStr::EqualTo("foo".into());
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchString>(r#"
/// "foo"
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchString>(r#"
/// "foo"
/// #   "#).unwrap());
/// # }
///
/// // -------------------
///
/// # {
/// #   let expected = MatchStr::Not(Box::new("bar".to_owned().into()));
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchString>(r#"
/// {"not": "bar"}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchString>(r#"
/// not: "bar"
/// #   "#).unwrap());
/// # }
///
/// // -------------------
///
/// # {
/// #   let expected = MatchStr::Or(vec![
/// #     MatchStr::Not(MatchStr::Contains("bar".into()).into()),
/// #     "foobar".to_owned().into(),
/// #   ]);
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchString>(r#"
/// {
///   "or": [
///     {"not": {"contains": "bar"}},
///     "foobar"
///   ]
/// }
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchString>(r#"
/// or:
///   - not:
///       contains: "bar"
///   - "foobar"
/// #   "#).unwrap());
/// # }
///
/// // -------------------
///
/// # {
/// #   let expected = MatchStr::Regex("fo{2,}".into());
/// // JSON
/// #   assert_eq!(expected, serde_json::from_str::<MatchString>(r#"
/// {"regex": "fo{2,}"}
/// #   "#).unwrap());
///
/// // YAML
/// #   assert_eq!(expected, serde_yaml::from_str::<MatchString>("
/// regex: 'fo{2,}'
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
/// # use winvoice_match::MatchStr;
/// # type MatchString = MatchStr<String>;
/// # let expected = MatchStr::Not(MatchStr::Any.into());
/// // JSON
/// # assert_eq!(expected, serde_json::from_str::<MatchString>(r#"
/// {"not": "any"}
/// # "#).unwrap());
///
/// // YAML
/// # assert_eq!(expected, serde_yaml::from_str::<MatchString>("
/// not: any
/// # ").unwrap());
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "snake_case"))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchStr<T>
{
	/// Match IFF all contained [`MatchStr`]s also match.
	And(Vec<Self>),

	/// Always match.
	Any,

	/// Match IFF some string `s` is partially equal to the contained value (e.g. "foo" contains
	/// "oo").
	Contains(T),

	/// Match IFF the contained [`MatchStr`] does _not_ match.
	Not(Box<Self>),

	/// Match IFF any contained [`MatchStr`] matches.
	Or(Vec<Self>),

	/// Match IFF some string `s` is described by this value when interpreted as a regular
	/// expression.
	///
	/// # Warnings
	///
	/// The syntax of a regular expression is highly dependent on the adapter which is being used:
	///
	/// * [Postgres](https://www.postgresql.org/docs/current/functions-matching.html#FUNCTIONS-POSIX-TABLE)
	Regex(T),

	/// Match IFF some string `s` matches the contained value.
	#[cfg_attr(feature = "serde", serde(untagged))]
	EqualTo(T),
}

impl<T> MatchStr<T>
{
	/// Combine this condition with some `other` condition using [`MatchStr::And`].
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::MatchStr as M;
	///
	/// assert_eq!(
	///   M::Any.and("1".into()).and("2".into()).and("3".into()),
	///   M::And(vec!["1".into(), "2".into(), "3".into()]),
	/// );
	/// ```
	///
	/// # See also
	///
	/// * [`MatchStr::and_mut`]
	pub fn and(mut self, other: Self) -> Self
	{
		self.and_mut(other);
		self
	}

	/// Combine this condition with some `other` condition using [`MatchStr::And`].
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::MatchStr as M;
	///
	/// let mut cond = M::Any;
	/// cond.and_mut("1".into());
	/// assert_eq!(cond, M::EqualTo("1"));
	///
	/// cond.and_mut("2".into());
	/// assert_eq!(cond, M::And(vec!["1".into(), "2".into()]));
	///
	/// cond.and_mut("3".into());
	/// assert_eq!(cond, M::And(vec!["1".into(), "2".into(), "3".into()]));
	/// ```
	pub fn and_mut(&mut self, other: Self)
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

	/// Transform some [`MatchStr`] of type `T` into another type `U` by providing a mapping
	/// `f`unction.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	///
	/// # Examples
	///
	/// ```rust
	/// use winvoice_match::MatchStr;
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   MatchStr::EqualTo("5").map(|s| s.to_string()),
	///   MatchStr::EqualTo("5".to_string())
	/// );
	/// ```
	pub fn map<F, MapTo>(self, f: F) -> MatchStr<MapTo>
	where
		F: Copy + Fn(T) -> MapTo,
	{
		match self
		{
			Self::And(match_conditions) => MatchStr::And(match_conditions.into_iter().map(|m| m.map(f)).collect()),
			Self::Any => MatchStr::Any,
			Self::Contains(x) => MatchStr::Contains(f(x)),
			Self::EqualTo(x) => MatchStr::EqualTo(f(x)),
			Self::Not(match_condition) => MatchStr::Not(match_condition.map(f).into()),
			Self::Or(match_conditions) => MatchStr::Or(match_conditions.into_iter().map(|m| m.map(f)).collect()),
			Self::Regex(x) => MatchStr::Regex(f(x)),
		}
	}

	/// Transform some [`MatchStr`] of type `T` into another type `U` by providing a mapping
	/// function.
	///
	/// # See also
	///
	/// * [`MatchStr::map`]
	pub fn map_ref<F, MapTo>(&self, f: F) -> MatchStr<MapTo>
	where
		F: Copy + Fn(&T) -> MapTo,
	{
		match self
		{
			Self::And(match_conditions) => MatchStr::And(match_conditions.iter().map(|m| m.map_ref(f)).collect()),
			Self::Any => MatchStr::Any,
			Self::Contains(x) => MatchStr::Contains(f(x)),
			Self::EqualTo(x) => MatchStr::EqualTo(f(x)),
			Self::Not(match_condition) => MatchStr::Not(match_condition.map_ref(f).into()),
			Self::Or(match_conditions) => MatchStr::Or(match_conditions.iter().map(|m| m.map_ref(f)).collect()),
			Self::Regex(x) => MatchStr::Regex(f(x)),
		}
	}

	/// Combine this condition with some `other` condition using [`MatchStr::Or`].
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::MatchStr as M;
	///
	/// assert_eq!(
	///   M::Not(M::Any.into()).or("1".into()).or("2".into()).or("3".into()),
	///   M::Or(vec!["1".into(), "2".into(), "3".into()]),
	/// );
	/// ```
	///
	/// # See also
	///
	/// * [`MatchStr::or_mut`]
	pub fn or(mut self, other: Self) -> Self
	{
		self.or_mut(other);
		self
	}

	/// Combine this condition with some `other` condition using [`MatchStr::Or`].
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::MatchStr as M;
	///
	/// let mut cond = M::Not(M::Any.into());
	/// cond.or_mut("1".into());
	/// assert_eq!(cond, M::EqualTo("1"));
	///
	/// cond.or_mut("2".into());
	/// assert_eq!(cond, M::Or(vec!["1".into(), "2".into()]));
	///
	/// cond.or_mut("3".into());
	/// assert_eq!(cond, M::Or(vec!["1".into(), "2".into(), "3".into()]));
	/// ```
	pub fn or_mut(&mut self, other: Self)
	{
		match self
		{
			Self::Not(inner) if matches!(**inner, Self::Any) => *self = other,
			Self::Or(ref mut vec) => vec.push(other),
			_ =>
			{
				let taken = mem::take(self);
				*self = Self::Or(vec![taken, other])
			},
		}
	}
}
