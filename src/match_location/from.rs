use clinvoice_schema::Id;

use super::MatchLocation;
use crate::Match;

impl From<Id> for MatchLocation
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Match<Id>> for MatchLocation
{
	fn from(match_condition: Match<Id>) -> Self
	{
		Self { id: match_condition, ..Default::default() }
	}
}
