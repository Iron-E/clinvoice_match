use money2::{Currency, Exchange, ExchangeRates};

use super::Match;

impl<T> Exchange for Match<T>
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
			Self::EqualTo(value) | Self::GreaterThan(value) | Self::LessThan(value) =>
			{
				value.exchange_mut(currency, rates);
			},
			Self::InRange(lesser, greater) =>
			{
				lesser.exchange_mut(currency, rates);
				greater.exchange_mut(currency, rates);
			},
			Self::Not(condition) => condition.exchange_mut(currency, rates),
		};
	}
}
