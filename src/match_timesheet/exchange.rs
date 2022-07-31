use money2::{Currency, Exchange, ExchangeRates};

use super::MatchTimesheet;

impl Exchange for MatchTimesheet
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			expenses: self.expenses.exchange(currency, rates),
			job: self.job.exchange(currency, rates),
			..self
		}
	}
}

impl Exchange for &MatchTimesheet
{
	type Output = MatchTimesheet;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			employee: self.employee.clone(),
			expenses: (&self.expenses).exchange(currency, rates),
			id: self.id.clone(),
			job: (&self.job).exchange(currency, rates),
			time_begin: self.time_begin.clone(),
			time_end: self.time_end.clone(),
			work_notes: self.work_notes.clone(),
		}
	}
}
