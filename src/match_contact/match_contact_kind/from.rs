use winvoice_schema::ContactKind;

use super::{MatchContactKind, MatchLocation};

impl From<ContactKind> for MatchContactKind
{
	fn from(kind: ContactKind) -> Self
	{
		match kind
		{
			ContactKind::Address(a) => MatchContactKind::Address(a.into()),
			ContactKind::Email(e) => MatchContactKind::Email(e.into()),
			ContactKind::Other(e) => MatchContactKind::Other(e.into()),
			ContactKind::Phone(e) => MatchContactKind::Phone(e.into()),
		}
	}
}

impl From<MatchLocation> for MatchContactKind
{
	fn from(address: MatchLocation) -> Self
	{
		Self::Address(address)
	}
}
