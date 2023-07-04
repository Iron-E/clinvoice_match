use core::ops::Not;

use super::MatchStr;

impl<T> Not for MatchStr<T>
{
	type Output = Self;

	fn not(self) -> Self::Output
	{
		Self::Not(self.into())
	}
}
