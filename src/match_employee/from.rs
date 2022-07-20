use clinvoice_schema::Id;

use super::MatchEmployee;

impl From<Id> for MatchEmployee
{
	fn from(id: Id) -> Self
	{
		Self {
			id: id.into(),
			..Default::default()
		}
	}
}
