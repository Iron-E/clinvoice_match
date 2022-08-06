use clinvoice_schema::Id;

use super::MatchJob;
use crate::Match;

impl From<Id> for MatchJob
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Match<Id>> for MatchJob
{
	fn from(match_condition: Match<Id>) -> Self
	{
		Self { id: match_condition, ..Default::default() }
	}
}
