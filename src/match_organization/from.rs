use clinvoice_schema::Id;

use super::MatchOrganization;

impl From<Id> for MatchOrganization
{
	fn from(id: Id) -> Self
	{
		Self {
			id: id.into(),
			..Default::default()
		}
	}
}
