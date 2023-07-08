use super::MatchSet;

impl<T> FromIterator<T> for MatchSet<T>
{
	/// Joins the `iter` using [`MatchSet::And`]
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = T>,
	{
		Self::And(iter.into_iter().map(|t| t.into()).collect())
	}
}
