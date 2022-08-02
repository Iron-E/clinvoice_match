use money2::{Currency, Exchange, ExchangeRates};

use super::MatchExpense;

impl Exchange for MatchExpense
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self.cost.exchange_mut(currency, rates);
	}
}
