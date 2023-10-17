use winvoice_schema::Timesheet;

use super::{
	DateTime,
	Id,
	Match,
	MatchEmployee,
	MatchExpense,
	MatchJob,
	MatchOption,
	MatchSet,
	MatchStr,
	MatchTimesheet,
	Utc,
};

impl From<Id> for MatchTimesheet
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<DateTime<Utc>> for MatchTimesheet
{
	fn from(date: DateTime<Utc>) -> Self
	{
		Match::from(date).into()
	}
}

impl From<Match<Id>> for MatchTimesheet
{
	fn from(id: Match<Id>) -> Self
	{
		Self { id, ..Default::default() }
	}
}

impl From<Match<DateTime<Utc>>> for MatchTimesheet
{
	fn from(time_begin: Match<DateTime<Utc>>) -> Self
	{
		Self { time_begin, ..Default::default() }
	}
}

impl From<MatchEmployee> for MatchTimesheet
{
	fn from(employee: MatchEmployee) -> Self
	{
		Self { employee, ..Default::default() }
	}
}

impl From<MatchExpense> for MatchTimesheet
{
	fn from(expenses: MatchExpense) -> Self
	{
		MatchSet::from(expenses).into()
	}
}

impl From<MatchJob> for MatchTimesheet
{
	fn from(job: MatchJob) -> Self
	{
		Self { job, ..Default::default() }
	}
}

impl From<MatchOption<Match<DateTime<Utc>>>> for MatchTimesheet
{
	fn from(time_end: MatchOption<Match<DateTime<Utc>>>) -> Self
	{
		Self { time_end, ..Default::default() }
	}
}

impl From<MatchSet<MatchExpense>> for MatchTimesheet
{
	fn from(expenses: MatchSet<MatchExpense>) -> Self
	{
		Self { expenses, ..Default::default() }
	}
}

impl From<MatchStr<String>> for MatchTimesheet
{
	fn from(work_notes: MatchStr<String>) -> Self
	{
		Self { work_notes, ..Default::default() }
	}
}

impl From<Option<Match<DateTime<Utc>>>> for MatchTimesheet
{
	fn from(date_close: Option<Match<DateTime<Utc>>>) -> Self
	{
		MatchOption::from(date_close).into()
	}
}

impl From<String> for MatchTimesheet
{
	fn from(work_notes: String) -> Self
	{
		MatchStr::from(work_notes).into()
	}
}

impl From<Timesheet> for MatchTimesheet
{
	fn from(timesheet: Timesheet) -> Self
	{
		Self {
			employee: timesheet.employee.into(),
			expenses: timesheet.expenses.into_iter().map(Into::into).collect(),
			id: timesheet.id.into(),
			job: timesheet.job.into(),
			time_begin: timesheet.time_begin.into(),
			time_end: timesheet.time_end.map(Into::into).into(),
			work_notes: timesheet.work_notes.into(),
		}
	}
}
