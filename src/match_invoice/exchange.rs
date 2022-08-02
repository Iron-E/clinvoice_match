use money2::{Currency, Exchange, ExchangeRates};

use super::MatchInvoice;

impl Exchange for MatchInvoice
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self.hourly_rate.exchange_mut(currency, rates);
	}
}
