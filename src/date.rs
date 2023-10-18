//---------------------------------------------------------------------------------------------------- Use
use crate::weekday::Weekday;
use crate::month::Month;
use crate::day::Day;

//---------------------------------------------------------------------------------------------------- Date
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Eq,Ord,Hash)]
/// Calendar date
pub struct Date {
	year: u16,
	month: Month,
	day: Day,
}

/// Any year from `0` to `65,535`
pub type Year = u16;

//---------------------------------------------------------------------------------------------------- Impl
impl Date {
	#[inline]
	/// Create a new [`Date`] from numbers
	///
	/// ## Panics
	/// This function panics if:
	/// - `month == 0`
	/// - `month > 12`
	/// - `day == 0`
	/// - `day < 32`
	///
	/// ```rust,should_panic
	/// # use nichi::*;
	/// Date::new(2000, 0, 0);
	/// ```
	/// ```rust,should_panic
	/// # use nichi::*;
	/// Date::new(2000, 1, 32);
	/// ```
	/// ```rust,should_panic
	/// # use nichi::*;
	/// Date::new(2000, 13, 31);
	/// ```
	pub const fn new(year: u16, month: u8, day: u8) -> Self {
		assert!(month != 0, "month was 0");
		assert!(month < 13, "month was greater than 13");
		assert!(day != 0, "day was 0");
		assert!(day < 32, "day was greater than 31");

		Self { year, month: Month::new(month), day: Day::new(day) }
	}

	#[inline]
	/// Create a new [`Date`] from typed [`Month`] and [`Day`]
	///
	/// ```rust
	/// # use nichi::*;
	/// let date = Date::new_typed(
	/// 	2000,
	/// 	Month::December,
	/// 	Day::TwentyFifth
	/// );
	///
	/// // Christmas in the year 2000 was on a Monday.
	/// assert_eq!(date.weekday(), Weekday::Monday);
	/// ```
	pub const fn new_typed(year: u16, month: Month, day: Day) -> Self {
		Self { year, month, day }
	}

	#[inline]
	/// Receive the corresponding [`Weekday`] of this [`Date`].
	///
	/// This uses [Tomohiko Sakamoto's](https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Sakamoto's_methods) algorithm.
	///
	/// It is accurate for any [`Date`].
	///
	/// ```rust
	/// # use nichi::{Date,Weekday};
	/// // US Independence day was on a Monday.
	/// assert_eq!(Date::new(1776, 7, 1).weekday(), Weekday::Monday);
	///
	/// // Nintendo Switch was released on a Friday.
	/// assert_eq!(Date::new(2017, 3, 3).weekday(), Weekday::Friday);
	///
	/// // Christmas in 1999 was on a Saturday.
	/// assert_eq!(Date::new(1999, 12, 25).weekday(), Weekday::Saturday);
	///
	/// // A good album was released on a Monday.
	/// assert_eq!(Date::new(2018, 9, 24).weekday(), Weekday::Monday);
	/// ```
	pub const fn weekday(&self) -> Weekday {
		let month: isize = self.month.as_u8() as isize - 1;
		debug_assert!(month >= 0);
		debug_assert!(month < 12);

		let year = if month < 2 {
			self.year.saturating_sub(1)
		} else {
			self.year
		};

		const LUT: [u16; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

		let lut: u16 = unsafe { std::ptr::read(LUT.as_ptr().offset(month)) };
		debug_assert!(lut < 12);

		let weekday: u16 = (year + year/4 - year/100 + year/400 + lut + self.day as u16) % 7;
		debug_assert!(weekday < 7);

		const WEEK: [Weekday; 7] = [
			Weekday::Sunday,
			Weekday::Monday,
			Weekday::Tuesday,
			Weekday::Wednesday,
			Weekday::Thursday,
			Weekday::Friday,
			Weekday::Saturday,
		];

		unsafe { std::ptr::read(WEEK.as_ptr().offset(weekday as isize)) }
	}
}