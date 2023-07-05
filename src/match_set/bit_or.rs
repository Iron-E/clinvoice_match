use core::{
	mem,
	ops::{BitOr, BitOrAssign},
};

use super::MatchSet;

impl<T> BitOr for MatchSet<T>
{
	type Output = Self;

	/// Combine this condition with some `other` condition using [`MatchSet::Or`].
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::{Match as M, MatchSet as S};
	///
	/// assert_eq!(
	///   !S::Any | M::from(1).into() | M::from(2).into() | M::from(3).into(),
	///   S::Or(vec![M::from(1).into(), M::from(2).into(), M::from(3).into()])
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

impl<T> BitOrAssign for MatchSet<T>
{
	/// Combine this condition with some `other` condition using [`MatchSet::Or`].
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::{Match as M, MatchSet as S};
	///
	/// let mut cond = !S::Any;
	/// cond |= M::from(1).into();
	/// assert_eq!(cond, S::Contains(M::from(1).into()));
	///
	/// cond |= M::from(2).into();
	/// assert_eq!(cond, S::Or(vec![M::from(1).into(), M::from(2).into()]));
	///
	/// cond |= M::from(3).into();
	/// assert_eq!(cond, S::Or(vec![M::from(1).into(), M::from(2).into(), M::from(3).into()]));
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
