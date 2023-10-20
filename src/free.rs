//---------------------------------------------------------------------------------------------------- Use
use crate::{Date,Year,Month,Day,DaysInMonth,Weekday};

//---------------------------------------------------------------------------------------------------- Use
#[inline]
/// If `year` is a leap year
///
/// Works with any year within `i128` unlike [`Year::is_leap`]
///
/// ```rust
/// # use nichi::*;
/// assert!(is_leap(2020));
/// assert!(is_leap(2024));
/// assert!(!is_leap(2019));
/// assert!(!is_leap(2023));
/// ```
///
/// ## Algorithm
/// <https://howardhinnant.github.io/date_algorithms.html#is_leap>
pub const fn is_leap(year: i128) -> bool {
	year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[inline]
/// Get the last day of a month
///
/// Works with any year within `i128` unlike [`Year::days_in_month`]
///
/// A negative `year` represents BCE years, e.g, `-1` is `1 BCE`.
///
/// ```rust
/// # use nichi::*;
/// // Last day of January of 2000 was the 31st.
/// assert_eq!(
/// 	days_in_month(2000, Month::January),
/// 	DaysInMonth::ThirtyOne,
/// );
///
/// // Last day of February of 2020 was the 29th.
/// assert_eq!(
/// 	days_in_month(2020, Month::February),
/// 	DaysInMonth::TwentyNine,
/// );
/// ```
///
/// ## Algorithm
/// <https://howardhinnant.github.io/date_algorithms.html#last_day_of_month>
pub const fn days_in_month(year: i128, month: Month) -> DaysInMonth {
	use Month as M;
	match month {
		M::January|M::March|M::May|M::July|
		M::August|M::October |M::December => DaysInMonth::ThirtyOne,

		M::April|M::June|M::September|M::November => DaysInMonth::Thirty,

		M::February => if is_leap(year) {
			DaysInMonth::TwentyNine
		} else {
			DaysInMonth::TwentyEight
		}
	}
}