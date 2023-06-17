use winvoice_schema::Id;

use super::{Match, MatchDepartment, MatchEmployee, MatchStr};

impl From<bool> for MatchEmployee
{
	fn from(active: bool) -> Self
	{
		Match::from(active).into()
	}
}

impl From<Id> for MatchEmployee
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Match<bool>> for MatchEmployee
{
	fn from(active: Match<bool>) -> Self
	{
		Self { active, ..Default::default() }
	}
}

impl From<Match<Id>> for MatchEmployee
{
	fn from(id: Match<Id>) -> Self
	{
		Self { id, ..Default::default() }
	}
}

impl From<MatchDepartment> for MatchEmployee
{
	fn from(department: MatchDepartment) -> Self
	{
		Self { department, ..Default::default() }
	}
}

impl From<MatchStr<String>> for MatchEmployee
{
	fn from(name: MatchStr<String>) -> Self
	{
		Self { name, ..Default::default() }
	}
}

impl From<String> for MatchEmployee
{
	fn from(name: String) -> Self
	{
		MatchStr::from(name).into()
	}
}
