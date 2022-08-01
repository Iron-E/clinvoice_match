use money2::{Currency, Exchange, ExchangeRates};

use super::MatchExpense;

impl Exchange for MatchExpense
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			cost: self.cost.exchange(currency, rates),
			..self
		}
	}
}

impl Exchange for &MatchExpense
{
	type Output = <MatchExpense as Exchange>::Output;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			category: self.category.clone(),
			cost: (&self.cost).exchange(currency, rates),
			description: self.description.clone(),
			id: self.id.clone(),
			timesheet_id: self.timesheet_id.clone(),
		}
	}
}
