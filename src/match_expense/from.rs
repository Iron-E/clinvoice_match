use winvoice_schema::Id;

use super::{Match, MatchExpense, MatchStr, Money};

impl From<Id> for MatchExpense
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Match<Id>> for MatchExpense
{
	fn from(id: Match<Id>) -> Self
	{
		Self { id, ..Default::default() }
	}
}

impl From<Match<Money>> for MatchExpense
{
	fn from(cost: Match<Money>) -> Self
	{
		Self { cost, ..Default::default() }
	}
}

impl From<MatchStr<String>> for MatchExpense
{
	fn from(category: MatchStr<String>) -> Self
	{
		Self { category, ..Default::default() }
	}
}

impl From<Money> for MatchExpense
{
	fn from(cost: Money) -> Self
	{
		Match::from(cost).into()
	}
}

impl From<String> for MatchExpense
{
	fn from(category: String) -> Self
	{
		MatchStr::from(category).into()
	}
}
