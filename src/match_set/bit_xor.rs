use core::{
	mem,
	ops::{BitXor, BitXorAssign},
};

use super::MatchSet;

impl<T> BitXor for MatchSet<T>
where
	T: Clone,
{
	type Output = Self;

	/// Combine this condition with some `other` condition using [`MatchSet::And`] / [`MatchSet::Or`]..
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::{Match as M, MatchSet as S};
	///
	/// assert_eq!(
	///   !S::Any ^ M::from(1).into() ^ M::from(2).into(),
	///   (S::from(M::from(1)) | M::from(2).into()) & !(S::from(M::from(1)) & M::from(2).into()),
	/// );
	/// ```
	///
	/// # See also
	///
	/// * [`MatchSet::or_mut`]
	fn bitxor(mut self, rhs: Self) -> Self::Output
	{
		self ^= rhs;
		self
	}
}

impl<T> BitXorAssign for MatchSet<T>
where
	T: Clone,
{
	/// Combine this condition with some `rhs` using [`MatchSet::And`] / [`MatchSet::Or`].
	///
	/// # Example
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use winvoice_match::{Match as M, MatchSet as S};
	///
	/// let mut cond = !S::Any;
	/// cond ^= M::from(1).into();
	/// assert_eq!(cond, M::from(1).into());
	///
	/// cond ^= M::from(2).into();
	/// assert_eq!(cond, (S::from(M::from(1)) | M::from(2).into()) & !(S::from(M::from(1)) & M::from(2).into()));
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
