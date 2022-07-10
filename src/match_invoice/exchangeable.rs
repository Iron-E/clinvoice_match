use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::MatchInvoice;

impl Exchangeable for MatchInvoice
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			hourly_rate: self.hourly_rate.exchange(currency, rates),
			..self
		}
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			date_issued: self.date_issued.clone(),
			date_paid: self.date_paid.clone(),
			hourly_rate: self.hourly_rate.exchange_ref(currency, rates),
		}
	}
}
