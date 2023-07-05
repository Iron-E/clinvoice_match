use core::{
	mem,
	ops::{BitXor, BitXorAssign},
};

use super::Match;

impl<T> BitXor for Match<T>
where
	T: Clone,
{
	type Output = Self;

	/// Combine this condition with some `other` condition using [`Match::And`] / [`Match::Or`]..
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::Match as M;
	///
	/// assert_eq!(
	///   !M::Any ^ 1.into() ^ 2.into(),
	///   (M::from(1) | 2.into()) & !(M::from(1) & 2.into()),
	/// );
	/// ```
	///
	/// # See also
	///
	/// * [`Match::or_mut`]
	fn bitxor(mut self, rhs: Self) -> Self::Output
	{
		self ^= rhs;
		self
	}
}

impl<T> BitXorAssign for Match<T>
where
	T: Clone,
{
	/// Combine this condition with some `rhs` using [`Match::And`] / [`Match::Or`].
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::Match as M;
	///
	/// let mut cond = !M::Any;
	/// cond ^= 1.into();
	/// assert_eq!(cond, M::EqualTo(1));
	///
	/// cond ^= 2.into();
	/// assert_eq!(cond, (M::from(1) | 2.into()) & !(M::from(1) & 2.into()));
	/// ```
	fn bitxor_assign(&mut self, rhs: Self)
	{
		match self
		{
			Self::Not(inner) if matches!(**inner, Self::Any) => *self = rhs,
			_ =>
			{
				let v = vec![mem::take(self), rhs];
				*self = Self::Or(v.clone()) & !Self::And(v);
			},
		}
	}
}
