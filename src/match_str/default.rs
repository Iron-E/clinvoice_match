use super::MatchStr;

impl<T> Default for MatchStr<T>
{
	fn default() -> Self
	{
		Self::Any
	}
}
