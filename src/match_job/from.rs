use clinvoice_schema::Id;

use super::MatchJob;

impl From<Id> for MatchJob
{
	fn from(id: Id) -> Self
	{
		Self {
			id: id.into(),
			..Default::default()
		}
	}
}
