use money2::{Currency, Exchange, ExchangeRates};

use super::Match;

impl<T> Exchange for Match<T>
where
	T: Exchange<Output = T>,
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		self.map(|e| e.exchange(currency, rates))
	}
}

impl<T> Exchange for &Match<T>
where
	for<'any> &'any T: Exchange<Output = T>,
{
	type Output = Match<T>;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		self.map_ref(|e| e.exchange(currency, rates))
	}
}
