use money2::{Currency, Exchange, ExchangeRates};

use super::MatchJob;

impl Exchange for MatchJob
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self.invoice.exchange_mut(currency, rates);
	}
}
