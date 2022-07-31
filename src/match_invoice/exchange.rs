use money2::{Currency, Exchange, ExchangeRates};

use super::MatchInvoice;

impl Exchange for MatchInvoice
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			hourly_rate: self.hourly_rate.exchange(currency, rates),
			..self
		}
	}
}

impl Exchange for &MatchInvoice
{
	type Output = MatchInvoice;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			date_issued: self.date_issued.clone(),
			date_paid: self.date_paid.clone(),
			hourly_rate: (&self.hourly_rate).exchange(currency, rates),
		}
	}
}
