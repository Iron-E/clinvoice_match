use clinvoice_schema::Id;

use super::MatchLocation;

impl From<Id> for MatchLocation
{
	fn from(id: Id) -> Self
	{
		Self {
			id: id.into(),
			..Default::default()
		}
	}
}
