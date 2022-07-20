use clinvoice_schema::Id;

use super::MatchExpense;
use crate::Match;

impl From<Id> for MatchExpense
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Match<Id>> for MatchExpense
{
	fn from(match_condition: Match<Id>) -> Self
	{
		Self {
			id: match_condition,
			..Default::default()
		}
	}
}
