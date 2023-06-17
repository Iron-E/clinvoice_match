use winvoice_schema::Id;

use super::{Match, MatchDepartment, MatchStr};

impl From<Id> for MatchDepartment
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Match<Id>> for MatchDepartment
{
	fn from(id: Match<Id>) -> Self
	{
		Self { id, ..Default::default() }
	}
}

impl From<MatchStr<String>> for MatchDepartment
{
	fn from(name: MatchStr<String>) -> Self
	{
		Self { name, ..Default::default() }
	}
}

impl From<String> for MatchDepartment
{
	fn from(s: String) -> Self
	{
		MatchStr::from(s).into()
	}
}
