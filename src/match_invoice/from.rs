use winvoice_schema::Invoice;

use super::{DateTime, Match, MatchInvoice, MatchOption, Money, Utc};

impl From<Invoice> for MatchInvoice
{
	fn from(invoice: Invoice) -> Self
	{
		Self {
			date_issued: invoice.date.map(|d| d.issued.into()).into(),
			date_paid: invoice.date.and_then(|d| d.paid.map(Into::into)).into(),
			hourly_rate: invoice.hourly_rate.into(),
		}
	}
}

impl From<Match<Money>> for MatchInvoice
{
	fn from(hourly_rate: Match<Money>) -> Self
	{
		Self { hourly_rate, ..Default::default() }
	}
}

impl From<Match<DateTime<Utc>>> for MatchInvoice
{
	fn from(date_issued: Match<DateTime<Utc>>) -> Self
	{
		MatchOption::Some(date_issued).into()
	}
}

impl From<MatchOption<Match<DateTime<Utc>>>> for MatchInvoice
{
	fn from(date_issued: MatchOption<Match<DateTime<Utc>>>) -> Self
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
