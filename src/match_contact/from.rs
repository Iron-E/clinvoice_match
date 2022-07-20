use clinvoice_schema::Id;

use super::MatchContact;

impl From<String> for MatchContact
{
	fn from(s: String) -> Self
	{
		Self {
			label: s.into(),
			..Default::default()
		}
	}
}
