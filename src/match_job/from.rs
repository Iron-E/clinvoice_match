use super::{
	Duration,
	Id,
	Match,
	MatchDepartment,
	MatchInvoice,
	MatchJob,
	MatchOption,
	MatchOrganization,
	MatchSet,
	MatchStr,
	NaiveDateTime,
	Serde,
};

impl From<Duration> for MatchJob
{
	fn from(duration: Duration) -> Self
	{
		Match::from(Serde::from(duration)).into()
	}
}

impl From<Id> for MatchJob
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<NaiveDateTime> for MatchJob
{
	fn from(date: NaiveDateTime) -> Self
	{
		Match::from(date).into()
	}
}

impl From<Match<Id>> for MatchJob
{
	fn from(id: Match<Id>) -> Self
	{
		Self { id, ..Default::default() }
	}
}

impl From<Match<NaiveDateTime>> for MatchJob
{
	fn from(date_open: Match<NaiveDateTime>) -> Self
	{
		Self { date_open, ..Default::default() }
	}
}

impl From<Match<Serde<Duration>>> for MatchJob
{
	fn from(increment: Match<Serde<Duration>>) -> Self
	{
		Self { increment, ..Default::default() }
	}
}

impl From<MatchDepartment> for MatchJob
{
	fn from(departments: MatchDepartment) -> Self
	{
		MatchSet::from(departments).into()
	}
}

impl From<MatchInvoice> for MatchJob
{
	fn from(invoice: MatchInvoice) -> Self
	{
		Self { invoice, ..Default::default() }
	}
}

impl From<MatchOrganization> for MatchJob
{
	fn from(client: MatchOrganization) -> Self
	{
		Self { client, ..Default::default() }
	}
}

impl From<MatchOption<Match<NaiveDateTime>>> for MatchJob
{
	fn from(date_close: MatchOption<Match<NaiveDateTime>>) -> Self
	{
		Self { date_close, ..Default::default() }
	}
}

impl From<MatchSet<MatchDepartment>> for MatchJob
{
	fn from(departments: MatchSet<MatchDepartment>) -> Self
	{
		Self { departments, ..Default::default() }
	}
}

impl From<MatchStr<String>> for MatchJob
{
	fn from(objectives: MatchStr<String>) -> Self
	{
		Self { objectives, ..Default::default() }
	}
}

impl From<Option<Match<NaiveDateTime>>> for MatchJob
{
	fn from(date_close: Option<Match<NaiveDateTime>>) -> Self
	{
		MatchOption::from(date_close).into()
	}
}
