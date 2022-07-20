use clinvoice_schema::Id;

use super::MatchEmployee;
use crate::Match;

impl From<Id> for MatchEmployee
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Match<Id>> for MatchEmployee
{
	fn from(match_condition: Match<Id>) -> Self
	{
		Self {
			id: match_condition,
			..Default::default()
		}
	}
}
