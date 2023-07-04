use core::ops::Not;

use super::MatchSet;

impl<T> Not for MatchSet<T>
{
	type Output = Self;

	fn not(self) -> Self::Output
	{
		Self::Not(self.into())
	}
}
