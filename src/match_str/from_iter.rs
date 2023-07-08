use super::MatchStr;

impl<T> FromIterator<T> for MatchStr<T>
{
	/// Joins the `iter` using [`Match::Or`]
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = T>,
	{
		Self::Or(iter.into_iter().map(Into::into).collect())
	}
}
