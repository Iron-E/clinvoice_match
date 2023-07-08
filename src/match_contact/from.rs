use winvoice_schema::Contact;

use super::{MatchContact, MatchStr};

impl From<Contact> for MatchContact
{
	fn from(contact: Contact) -> Self
	{
		Self { kind: contact.kind.into(), label: contact.label.into() }
	}
}

impl From<MatchStr<String>> for MatchContact
{
	fn from(match_condition: MatchStr<String>) -> Self
	{
		Self { label: match_condition, ..Default::default() }
	}
}

impl From<String> for MatchContact
{
	fn from(s: String) -> Self
	{
		MatchStr::from(s).into()
	}
}
