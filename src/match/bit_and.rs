use core::{
	mem,
	ops::{BitAnd, BitAndAssign},
};

use super::Match;

impl<T> BitAnd for Match<T>
{
	type Output = Self;

	/// Combine this condition with some `other` condition using [`Match::And`].
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::Match as M;
	///
	/// assert_eq!(
	///   M::Any & 1.into() & 2.into() & 3.into(),
	///   M::And(vec![1.into(), 2.into(), 3.into()]),
	/// );
	/// ```
	///
	/// # See also
	///
	/// * [`Match::and_mut`]
	fn bitand(mut self, rhs: Self) -> Self::Output
	{
		self &= rhs;
		self
	}
}

impl<T> BitAndAssign for Match<T>
{
	/// Combine this condition with some `rhs` using [`Match::And`].
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::Match as M;
	///
	/// let mut cond = M::Any;
	/// cond &= 1.into();
	/// assert_eq!(cond, M::EqualTo(1));
	///
	/// cond &= 2.into();
	/// assert_eq!(cond, M::And(vec![1.into(), 2.into()]));
	///
	/// cond &= 3.into();
	/// assert_eq!(cond, M::And(vec![1.into(), 2.into(), 3.into()]));
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
