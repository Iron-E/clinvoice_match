use winvoice_schema::{Expense, Id};

use super::{Match, MatchExpense, MatchStr, Money};

impl From<Expense> for MatchExpense
{
	fn from(expense: Expense) -> Self
	{
		Self {
			category: expense.category.into(),
			cost: expense.cost.into(),
			description: expense.description.into(),
			id: expense.id.into(),
			timesheet_id: expense.timesheet_id.into(),
		}
	}
}

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
