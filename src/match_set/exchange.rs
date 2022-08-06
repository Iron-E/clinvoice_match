use money2::{Currency, Exchange, ExchangeRates};

use super::MatchSet;

impl<T> Exchange for MatchSet<T>
where
	T: Exchange,
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		match self
		{
			Self::And(conditions) | Self::Or(conditions) =>
			{
				conditions.exchange_mut(currency, rates)
			},
			Self::Any => (),
			Self::Contains(value) => value.exchange_mut(currency, rates),
			Self::Not(condition) => condition.exchange_mut(currency, rates),
		};
	}
}
