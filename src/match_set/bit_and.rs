use core::{
	mem,
	ops::{BitAnd, BitAndAssign},
};

use super::MatchSet;

impl<T> BitAnd for MatchSet<T>
{
	type Output = Self;

	/// Combine this condition with some `other` condition using [`MatchSet::And`].
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::{Match as M, MatchSet as S};
	///
	/// assert_eq!(
	///   S::Any & M::from(1).into() & M::from(2).into() & M::from(3).into(),
	///   S::And(vec![M::from(1).into(), M::from(2).into(), M::from(3).into()])
	/// );
	/// ```
	///
	/// # See also
	///
	/// * [`MatchStr::or_mut`]
	fn bitand(mut self, rhs: Self) -> Self::Output
	{
		self &= rhs;
		self
	}
}

impl<T> BitAndAssign for MatchSet<T>
{
	/// Combine this condition with some `other` condition using [`MatchSet::And`].
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::{Match as M, MatchSet as S};
	///
	/// let mut cond = S::Any;
	/// cond &= M::from(1).into();
	/// assert_eq!(cond, S::Contains(M::from(1).into()));
	///
	/// cond &= M::from(2).into();
	/// assert_eq!(cond, S::And(vec![M::from(1).into(), M::from(2).into()]));
	///
	/// cond &= M::from(3).into();
	/// assert_eq!(cond, S::And(vec![M::from(1).into(), M::from(2).into(), M::from(3).into()]));
	/// ```
	fn bitand_assign(&mut self, rhs: Self)
	{
		match self
		{
			Self::Any => *self = rhs,
			Self::And(ref mut vec) => vec.push(rhs),
			_ => *self = Self::And(vec![mem::take(self), rhs]),
		}
	}
}
