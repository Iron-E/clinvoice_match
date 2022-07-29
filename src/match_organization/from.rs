use clinvoice_schema::Id;

use super::MatchOrganization;
use crate::Match;

impl From<Id> for MatchOrganization
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Match<Id>> for MatchOrganization
{
	fn from(match_condition: Match<Id>) -> Self
	{
		Self {
			id: match_condition,
			..Default::default()
		}
	}
}
