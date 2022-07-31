use money2::{Currency, Exchange, ExchangeRates};

use super::MatchJob;

impl Exchange for MatchJob
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			invoice: self.invoice.exchange(currency, rates),
			..self
		}
	}
}

impl Exchange for &MatchJob
{
	type Output = MatchJob;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			client: self.client.clone(),
			date_close: self.date_close.clone(),
			date_open: self.date_open.clone(),
			id: self.id.clone(),
			increment: self.increment.clone(),
			invoice: (&self.invoice).exchange(currency, rates),
			notes: self.notes.clone(),
			objectives: self.objectives.clone(),
		}
	}
}
