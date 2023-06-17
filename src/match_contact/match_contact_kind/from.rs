use super::{MatchContactKind, MatchLocation};

impl From<MatchLocation> for MatchContactKind
{
	fn from(address: MatchLocation) -> Self
	{
		Self::Address(address)
	}
}
