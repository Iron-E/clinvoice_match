use core::{
	mem,
	ops::{BitOr, BitOrAssign},
};

use super::MatchStr;

impl<T> BitOr for MatchStr<T>
{
	type Output = Self;

	/// Combine this condition with some `other` condition using [`MatchStr::Or`].
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::MatchStr as M;
	///
	/// assert_eq!(
	///   !M::Any | "1".into() | "2".into() | "3".into(),
	///   M::Or(vec!["1".into(), "2".into(), "3".into()]),
	/// );
	/// ```
	///
	/// # See also
	///
	/// * [`MatchStr::or_mut`]
	fn bitor(mut self, rhs: Self) -> Self::Output
	{
		self |= rhs;
		self
	}
}

impl<T> BitOrAssign for MatchStr<T>
{
	/// Combine this condition with some `rhs` using [`MatchStr::Or`].
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::MatchStr as M;
	///
	/// let mut cond = !M::Any;
	/// cond |= "1".into();
	/// assert_eq!(cond, M::EqualTo("1"));
	///
	/// cond |= "2".into();
	/// assert_eq!(cond, M::Or(vec!["1".into(), "2".into()]));
	///
	/// cond |= "3".into();
	/// assert_eq!(cond, M::Or(vec!["1".into(), "2".into(), "3".into()]));
	/// ```
	fn bitor_assign(&mut self, rhs: Self)
	{
		match self
		{
			Self::Not(inner) if matches!(**inner, Self::Any) => *self = rhs,
			Self::Or(ref mut vec) => vec.push(rhs),
			_ => *self = Self::Or(vec![mem::take(self), rhs]),
		}
	}
}
