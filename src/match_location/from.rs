use winvoice_schema::{Id, Location};

use super::{Currency, Match, MatchLocation, MatchOption, MatchStr};

impl From<Id> for MatchLocation
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Location> for MatchLocation
{
	fn from(location: Location) -> Self
	{
		Self {
			currency: location.currency.map(Match::from).into(),
			id: location.id.into(),
			name: location.name.into(),
			outer: location.outer.map(|o| Box::new((*o).into())).into(),
		}
	}
}

impl From<Match<Id>> for MatchLocation
{
	fn from(id: Match<Id>) -> Self
	{
		Self { id, ..Default::default() }
	}
}

impl From<MatchOption<Match<Currency>>> for MatchLocation
{
	fn from(currency: MatchOption<Match<Currency>>) -> Self
	{
		Self { currency, ..Default::default() }
	}
}

impl From<MatchOption<Box<MatchLocation>>> for MatchLocation
{
	fn from(outer: MatchOption<Box<MatchLocation>>) -> Self
	{
		Self { outer, ..Default::default() }
	}
}

impl From<MatchStr<String>> for MatchLocation
{
	fn from(name: MatchStr<String>) -> Self
	{
		Self { name, ..Default::default() }
	}
}

impl From<Option<Box<MatchLocation>>> for MatchLocation
{
	fn from(outer: Option<Box<MatchLocation>>) -> Self
	{
		MatchOption::from(outer).into()
	}
}

impl From<Option<Match<Currency>>> for MatchLocation
{
	fn from(match_condition: Option<Match<Currency>>) -> Self
	{
		MatchOption::from(match_condition).into()
	}
}

impl From<Option<MatchLocation>> for MatchLocation
{
	fn from(outer: Option<MatchLocation>) -> Self
	{
		outer.map(Box::new).into()
	}
}

impl From<String> for MatchLocation
{
	fn from(name: String) -> Self
	{
		MatchStr::from(name).into()
	}
}
