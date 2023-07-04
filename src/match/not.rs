use core::ops::Not;

use super::Match;

impl<T> Not for Match<T>
{
	type Output = Self;

	fn not(self) -> Self::Output
	{
		Self::Not(self.into())
	}
}
