use winvoice_schema::ContactKind;

use super::{MatchContactKind, MatchLocation};

impl From<ContactKind> for MatchContactKind
{
	fn from(kind: ContactKind) -> Self
	{
		match kind
		{
			ContactKind::Address(a) => Self::Address(a.into()),
			ContactKind::Email(e) => Self::Email(e.into()),
			ContactKind::Other(e) => Self::Other(e.into()),
			ContactKind::Phone(e) => Self::Phone(e.into()),
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
