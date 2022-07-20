use super::MatchContact;
use crate::MatchStr;

impl From<String> for MatchContact
{
	fn from(s: String) -> Self
	{
		MatchStr::from(s).into()
	}
}

impl From<MatchStr<String>> for MatchContact
{
	fn from(match_condition: MatchStr<String>) -> Self
	{
		Self {
			label: match_condition,
			..Default::default()
		}
	}
}
