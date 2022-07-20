use clinvoice_schema::Id;

use super::MatchTimesheet;

impl From<Id> for MatchTimesheet
{
	fn from(id: Id) -> Self
	{
		Self {
			id: id.into(),
			..Default::default()
		}
	}
}
