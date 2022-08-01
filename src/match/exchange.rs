use money2::{Currency, Exchange, ExchangeRates};

use super::Match;

impl<T> Exchange for Match<T>
where
	T: Exchange,
{
	type Output = Match<T::Output>;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		self.map(|e| e.exchange(currency, rates))
	}
}

impl<T, U> Exchange for &Match<T>
where
	for<'any> &'any T: Exchange<Output = U>,
{
	type Output = Match<U>;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		self.map_ref(|e| e.exchange(currency, rates))
	}
}
