use super::{Id, Match, MatchLocation, MatchOrganization, MatchStr};

impl From<Id> for MatchOrganization
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Match<Id>> for MatchOrganization
{
	fn from(id: Match<Id>) -> Self
	{
		Self { id, ..Default::default() }
	}
}

impl From<MatchLocation> for MatchOrganization
{
	fn from(location: MatchLocation) -> Self
	{
		Self { location, ..Default::default() }
	}
}

impl From<MatchStr<String>> for MatchOrganization
{
	fn from(name: MatchStr<String>) -> Self
	{
		Self { name, ..Default::default() }
	}
}

impl From<String> for MatchOrganization
{
	fn from(name: String) -> Self
	{
		MatchStr::from(name).into()
	}
}
