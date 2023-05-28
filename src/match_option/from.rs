use super::MatchOption;

impl<T> From<Option<T>> for MatchOption<T>
{
	fn from(t: Option<T>) -> Self
	{
		t.map_or(Self::None, Self::Some)
	}
}
