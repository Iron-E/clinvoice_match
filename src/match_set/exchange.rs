use money2::{Currency, Exchange, ExchangeRates};

use super::MatchSet;

impl<T> Exchange for MatchSet<T>
where
	T: Exchange<Output = T>,
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		self.map(|e| e.exchange(currency, rates))
	}
}

impl<T> Exchange for &MatchSet<T>
where
	for<'any> &'any T: Exchange<Output = T>,
{
	type Output = MatchSet<T>;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		self.map_ref(|e| e.exchange(currency, rates))
	}
}
