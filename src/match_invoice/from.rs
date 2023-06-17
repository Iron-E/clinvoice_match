use super::{Match, MatchInvoice, MatchOption, Money, NaiveDateTime};

impl From<Match<Money>> for MatchInvoice
{
	fn from(hourly_rate: Match<Money>) -> Self
	{
		Self { hourly_rate, ..Default::default() }
	}
}

impl From<Match<NaiveDateTime>> for MatchInvoice
{
	fn from(date_issued: Match<NaiveDateTime>) -> Self
	{
		MatchOption::Some(date_issued).into()
	}
}

impl From<MatchOption<Match<NaiveDateTime>>> for MatchInvoice
{
	fn from(date_issued: MatchOption<Match<NaiveDateTime>>) -> Self
	{
		Self { date_issued, ..Default::default() }
	}
}

impl From<Money> for MatchInvoice
{
	fn from(hourly_rate: Money) -> Self
	{
		Match::from(hourly_rate).into()
	}
}
