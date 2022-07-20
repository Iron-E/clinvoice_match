use clinvoice_schema::Id;

use super::MatchExpense;

impl From<Id> for MatchExpense
{
	fn from(id: Id) -> Self
	{
		Self {
			id: id.into(),
			..Default::default()
		}
	}
}
