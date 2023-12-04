//---------------------------------------------------------------------------------------------------- Use
use crate::weekday::Weekday;
use crate::year::Year;
use crate::month::Month;
use crate::day::Day;
use once_cell::sync::Lazy;
use regex::Regex;

//---------------------------------------------------------------------------------------------------- Date
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Eq,Ord,Hash)]
/// Calendar date
pub struct Date {
	year: Year,
	month: Month,
	day: Day,
}

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
	pub const fn new(year: i16, month: u8, day: u8) -> Self {
		assert!(month != 0, "month was 0");
		assert!(month < 13, "month was greater than 13");
		assert!(day != 0, "day was 0");
		assert!(day < 32, "day was greater than 31");

		Self { year: Year(year), month: Month::new(month), day: Day::new(day) }
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// let date = Date::new_saturating(2000, 0, 0);
	/// assert_eq!(date.inner(), (2000, 1, 1));
	///
	/// let date = Date::new_saturating(2000, 13, 32);
	/// assert_eq!(date.inner(), (2000, 12, 31));
	/// ```
	pub const fn new_saturating(year: i16, month: u8, day: u8) -> Self {
		Self { year: Year(year), month: Month::new_saturating(month), day: Day::new_saturating(day) }
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// // Year does not wrap.
	///
	/// let date = Date::new_wrapping(2000, 0, 0);
	/// assert_eq!(date.inner(), (2000, 12, 31));
	///
	/// let date = Date::new_wrapping(2000, 13, 32);
	/// assert_eq!(date.inner(), (2000, 1, 1));
	/// ```
	pub const fn new_wrapping(year: i16, month: u8, day: u8) -> Self {
		Self { year: Year(year), month: Month::new_wrapping(month), day: Day::new_wrapping(day) }
	}

	#[inline]
	/// Create a new [`Date`] from typed [`Year`], [`Month`], and [`Day`]
	///
	/// ```rust
	/// # use nichi::*;
	/// let date = Date::new_typed(
	/// 	Year(2000),
	/// 	Month::December,
	/// 	Day::TwentyFifth
	/// );
	///
	/// // Christmas in the year 2000 was on a Monday.
	/// assert_eq!(date.weekday(), Weekday::Monday);
	/// ```
	pub const fn new_typed(year: Year, month: Month, day: Day) -> Self {
		Self { year, month, day }
	}

	#[inline]
	/// Receive the corresponding [`Weekday`] of this [`Date`].
	///
	/// It is accurate for any [`Date`].
	///
	/// ```rust
	/// # use nichi::{Date,Weekday};
	/// // US Independence day was on a Thursday.
	/// assert_eq!(Date::new(1776, 7, 4).weekday(), Weekday::Thursday);
	///
	/// // Nintendo Switch was released on a Friday.
	/// assert_eq!(Date::new(2017, 3, 3).weekday(), Weekday::Friday);
	///
	/// // Christmas in 1999 was on a Saturday.
	/// assert_eq!(Date::new(1999, 12, 25).weekday(), Weekday::Saturday);
	///
	/// // A good album was released on a Wednesday.
	/// assert_eq!(Date::new(2018, 4, 25).weekday(), Weekday::Wednesday);
	/// ```
	///
	/// ## Algorithm
	/// This uses [Tomohiko Sakamoto's](https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Sakamoto's_methods) algorithm.
	pub const fn weekday(self) -> Weekday {
		Self::weekday_raw(self.year.inner(), self.month.inner(), self.day.inner())
	}

	#[inline]
	/// Same as [`Date::weekday`] but with raw number primitives
	pub const fn weekday_raw(year: i16, month: u8, day: u8) -> Weekday {
		let month: isize = month as isize - 1;
		debug_assert!(month >= 0);
		debug_assert!(month < 12);

		let year = if month < 2 {
			year.saturating_sub(1)
		} else {
			year
		};

		const LUT: [i16; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

		// SAFETY: indexing is not const, so we must
		// "index" by using pointer offsetting.
		let lut: i16 = unsafe { std::ptr::read(LUT.as_ptr().offset(month)) };
		debug_assert!(lut < 12);

		let weekday: i16 = (year + year/4 - year/100 + year/400 + lut + day as i16) % 7;
		debug_assert!(weekday < 7);

		// SAFETY: indexing is not const, so we must
		// "index" by using pointer offsetting.
		unsafe { std::ptr::read(Weekday::ALL.as_ptr().offset(weekday as isize)) }
	}

	/// ```rust
	/// # use nichi::*;
	/// let date = Date::new(2000, 12, 25);
	/// let ((year, month, day)) = date.inner();
	///
	/// assert_eq!((year, month, day), (2000, 12, 25));
	/// ```
	pub const fn inner(self) -> (i16, u8, u8) {
		(self.year.inner(), self.month.inner(), self.day.inner())
	}

	/// ```rust
	/// # use nichi::*;
	/// let date = Date::new(2000, 12, 25);
	/// assert_eq!(date.inner_typed(), (Year(2000), Month::new(12), Day::new(25)));
	/// ```
	pub const fn inner_typed(self) -> (Year, Month, Day) {
		(self.year, self.month, self.day)
	}

	/// ```rust
	/// # use nichi::*;
	/// let date = Date::new(2000, 12, 25);
	/// assert_eq!(date.year(), 2000);
	/// ```
	pub const fn year(self) -> Year {
		self.year
	}

	/// ```rust
	/// # use nichi::*;
	/// let date = Date::new(2000, 12, 25);
	/// assert_eq!(date.month(), 12);
	/// ```
	pub const fn month(self) -> Month {
		self.month
	}

	/// ```rust
	/// # use nichi::*;
	/// let date = Date::new(2000, 12, 25);
	/// assert_eq!(date.day(), 25);
	/// ```
	pub const fn day(self) -> Day {
		self.day
	}

	#[inline]
	/// Create [`Date`] from a string
	///
	/// ## Invariants
	/// - The year must be `1000..=9999`
	/// - The month must be at least the first 3 letters of the month in english (`oct`, `Dec`, `SEP`, etc)
	/// - The day must be a number, either optionally with a leading `0` or suffixed by `th`, `rd`, `nd`, `st` (but not both, e.g, `3rd` is OK, `03` is OK, `03rd` is INVALID)
	///
	/// The order of the `year`, `month`, and `day` do not matter:
	/// ```rust
	/// # use nichi::*;
	/// let december_25th_2010 = Date::new(2010, 12, 25);
	/// assert_eq!(Date::from_str("dec 25 2010").unwrap(), december_25th_2010);
	/// assert_eq!(Date::from_str("2010 dec 25").unwrap(), december_25th_2010);
	/// assert_eq!(Date::from_str("2010 25th Dec").unwrap(), december_25th_2010);
	/// assert_eq!(Date::from_str("25TH 2010 DEC").unwrap(), december_25th_2010);
	/// ```
	///
	/// Infinite amount of separator characters are allowed:
	/// ```rust
	/// # use nichi::*;
	/// let december_25th_2010 = Date::new(2010, 12, 25);
	/// assert_eq!(Date::from_str("dec-25 ...       2010").unwrap(), december_25th_2010);
	/// ```
	///
	/// This function is extremely lenient, as long as some resemblance of a
	/// calendar date is in the input string, it will parse it out:
	/// ```rust
	/// # use nichi::*;
	/// //                                            Year 2010
	/// //                                  25th day      |
	/// //                         December     |         |
	/// //                            |         |         |
	/// assert_eq!( //                v         v         v
	/// 	Date::from_str("----fasdf decBR wef 25 a - >.a2010a...aa").unwrap(),
	/// 	Date::new(2010, 12, 25),
	/// );
	/// ```
	///
	/// ## ISO 8601 (like)
	/// This function also parses `ISO 8601`-like dates.
	///
	/// The `year`, `month`, and `day` must be available in that order.
	///
	/// A single separator character must exist, although it does not need to be `-`.
	///
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Date::from_str("2010-12-25").unwrap(), Date::new(2010, 12, 25));
	/// assert_eq!(Date::from_str("2010.02.02").unwrap(), Date::new(2010, 2, 2));
	/// assert_eq!(Date::from_str("2010/2/2").unwrap(),   Date::new(2010, 2, 2));
	/// assert_eq!(Date::from_str("2010_02_2").unwrap(),  Date::new(2010, 2, 2));
	/// assert_eq!(Date::from_str("2010 2 02").unwrap(),  Date::new(2010, 2, 2));
	/// ```
	///
	/// ## Examples
	/// ```rust
	/// # use nichi::*;
	/// let december_25th_2010 = Date::new(2010, 12, 25);
	///
	/// assert_eq!(Date::from_str("dec, 25, 2010").unwrap(),        december_25th_2010);
	/// assert_eq!(Date::from_str("dec 25 2010").unwrap(),          december_25th_2010);
	/// assert_eq!(Date::from_str("Dec 25th 2010").unwrap(),        december_25th_2010);
	/// assert_eq!(Date::from_str("DEC 25TH 2010").unwrap(),        december_25th_2010);
	/// assert_eq!(Date::from_str("DEC-25th-2010").unwrap(),        december_25th_2010);
	/// assert_eq!(Date::from_str("2010.dec.25").unwrap(),          december_25th_2010);
	/// assert_eq!(Date::from_str("2010, 25th, Dec").unwrap(),      december_25th_2010);
	/// assert_eq!(Date::from_str("2010 december 25th").unwrap(),   december_25th_2010);
	/// assert_eq!(Date::from_str("2010, DECEMBER, 25th").unwrap(), december_25th_2010);
	/// assert_eq!(Date::from_str("DECEMBER 25th 2010").unwrap(),   december_25th_2010);
	/// assert_eq!(Date::from_str("December 25th, 2010").unwrap(),  december_25th_2010);
	///
	/// let april_3rd_1000 = Date::new(1000, 4, 3);
	/// assert_eq!(Date::from_str("apr, 3, 1000").unwrap(),     april_3rd_1000);
	/// assert_eq!(Date::from_str("apr 03 1000").unwrap(),      april_3rd_1000);
	/// assert_eq!(Date::from_str("Apr 3rd 1000").unwrap(),    april_3rd_1000);
	/// assert_eq!(Date::from_str("APR 3RD 1000").unwrap(),     april_3rd_1000);
	/// assert_eq!(Date::from_str("APR-3RD-1000").unwrap(),    april_3rd_1000);
	/// assert_eq!(Date::from_str("1000.apr.03").unwrap(),      april_3rd_1000);
	/// assert_eq!(Date::from_str("1000, 3rd, Apr").unwrap(),   april_3rd_1000);
	/// assert_eq!(Date::from_str("1000 april 3rd").unwrap(),  april_3rd_1000);
	/// assert_eq!(Date::from_str("1000, APRIL, 3RD").unwrap(), april_3rd_1000);
	/// assert_eq!(Date::from_str("APRIL 3rd 1000").unwrap(),   april_3rd_1000);
	/// assert_eq!(Date::from_str("April 3rd, 1000").unwrap(), april_3rd_1000);
	/// ```
	pub fn from_str(s: &str) -> Option<Self> {
		// Debug.
		// println!("{s}");

		// ISO 8601
		static ISO: Lazy<Regex> = Lazy::new(|| Regex::new(r"[1-9]\d{3}.\d{1,2}.\d{1,2}").unwrap());
		static ISO_1: Lazy<Regex> = Lazy::new(|| Regex::new(r"[1-9]\d{3}.(0[1-9]|1[012]).(0[1-9]|[12][0-9]|30|31)").unwrap());
		static ISO_2: Lazy<Regex> = Lazy::new(|| Regex::new(r"[1-9]\d{3}.[1-9].(0[1-9]|[12][0-9]|30|31)").unwrap());
		static ISO_3: Lazy<Regex> = Lazy::new(|| Regex::new(r"[1-9]\d{3}.(0[1-9]|1[012]).[1-9]").unwrap());
		static ISO_4: Lazy<Regex> = Lazy::new(|| Regex::new(r"[1-9]\d{3}.[1-9].[1-9]").unwrap());

		// If ISO matches, attempt that first.
		if ISO.is_match(s) {
			// Debug
			// println!("iso {s}");

			// xxxx-xx-xx
			if let Some(m) = ISO_1.find(s) {
				// println!("iso2 {m:?}");
				let s = m.as_str();
				let b = s.as_bytes();
				let year  = s[0..4].parse::<i16>().unwrap();
				let month = Month::from_bytes(&b[5..7]).unwrap();
				let day   = Day::from_bytes(&b[8..10]).unwrap();
				return Some(Self { year: Year(year), month, day })
			// xxxx-x-xx
			} else if let Some(m) = ISO_2.find(s) {
				// println!("iso2 {m:?}");
				let s = m.as_str();
				let b = s.as_bytes();
				let year  = s[0..4].parse::<i16>().unwrap();
				let month = Month::from_bytes(&b[5..6]).unwrap();
				let day   = Day::from_bytes(&b[7..9]).unwrap();
				return Some(Self { year: Year(year), month, day })
			// xxxx-xx-x
			} else if let Some(m) = ISO_3.find(s) {
				// println!("iso3 {m:?}");
				let s = m.as_str();
				let b = s.as_bytes();
				let year  = s[0..4].parse::<i16>().unwrap();
				let month = Month::from_bytes(&b[5..7]).unwrap();
				let day   = Day::from_bytes(&b[8..9]).unwrap();
				return Some(Self { year: Year(year), month, day })
			// xxxx-x-x
			} else if let Some(m) = ISO_4.find(s) {
				// println!("iso4 {m:?}");
				let s = m.as_str();
				let b = s.as_bytes();
				let year  = s[0..4].parse::<i16>().unwrap();
				let month = Month::from_bytes(&b[5..6]).unwrap();
				let day   = Day::from_bytes(&b[7..8]).unwrap();
				return Some(Self { year: Year(year), month, day })
			}

			unreachable!("iso format matches but could not be parsed");
		}

		// Year.
		static YEAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d{4}").unwrap());
		// Month.
		static MONTH: Lazy<Regex> = Lazy::new(|| Regex::new(
r"Jan|jan|JAN|Feb|feb|FEB|Mar|mar|MAR|Apr|apr|APR|Jun|jun|JUN|Jul|jul|JUL|Aug|aug|AUG|Sep|sep|SEP|Oct|oct|OCT|Nov|nov|NOV|Dec|dec|DEC"
		).unwrap());
		// Day.
		// 2 numbers followed by 2 [A-Za-z] OR 2 numbers
		static DAY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d{2}|\d{1})[A-Za-z]{2}|\b\d{2}\b|\b\d{1}\b").unwrap());

		// Attempt year.
		let Some(year) = YEAR.find(s) else {
			return None;
		};
		let Some(year) = Year::from_str(year.as_str()) else {
			return None;
		};

		// Attempt month.
		let Some(month) = MONTH.find(s) else {
			return None;
		};
		let Some(month) = Month::from_str(month.as_str()) else {
			return None;
		};

		// Attempt day.
		let Some(day) = DAY.find(s) else {
			return None;
		};
		let Some(day) = Day::from_str(day.as_str()) else {
			return None;
		};

		Some(Self { year, month, day })
	}

	#[inline]
	/// Convert a UNIX timestamp into a [`Date`]
	///
	/// This converts a UNIX timestamp into a calendar day, assuming UTC.
	///
	/// It is the reverse of [`Date::as_unix`].
	///
	/// The input is relative to the `UNIX_EPOCH`, for example:
	/// ```rust
	/// # use nichi::*;
	/// // 0 is the UNIX_EPOCH
	/// assert_eq!(Date::from_unix(0),     Date::new(1970, 1, 1));
	/// assert_eq!(Date::from_unix(86399), Date::new(1970, 1, 1));
	///
	/// // 1 day after.
	/// assert_eq!(Date::from_unix(86400), Date::new(1970, 1, 2));
	/// ```
	///
	/// A negative input will return dates before the `UNIX_EPOCH`:
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Date::from_unix(-1),     Date::new(1969, 12, 31));
	/// assert_eq!(Date::from_unix(-86400), Date::new(1969, 12, 30));
	/// ```
	///
	/// ## Example
	/// ```rust
	/// # use nichi::*;
	/// let date = Date::from_unix(1697760000);
	/// assert_eq!(date, Date::new(2023, 10, 20));
	///
	/// assert_eq!(date.as_unix(), 1697760000);
	/// ```
	///
	/// ## Algorithm
	/// <https://howardhinnant.github.io/date_algorithms.html#civil_from_days>
	pub const fn from_unix(seconds_relative_to_unix_epoch: i128) -> Self {
		let s = if seconds_relative_to_unix_epoch.is_negative() {
			seconds_relative_to_unix_epoch - 86400
		} else {
			seconds_relative_to_unix_epoch
		};

		let z:   i64 = ((s / 86400) + 719468) as i64;
		let era: i64 = if z >= 0 { z } else { z - 146096 } / 146097;
		let doe: u64 = (z - era * 146097) as u64;
		let yoe: u64 = (doe - doe/1460 + doe/36524 - doe/146096) / 365;
		let y:   i64 = (yoe as i64) + era * 400;
		let doy: u64 = doe - (365*yoe + yoe/4 - yoe/100);
		let mp:  u64 = (5*doy + 2)/153;
		let d:   u8  = (doy - (153*mp+2)/5 + 1) as u8;
		let m:   u8  = (if mp < 10 { mp + 3 } else { mp - 9 }) as u8;

		debug_assert!(m != 0);
		debug_assert!(m < 13);
		debug_assert!(d != 0);
		debug_assert!(d < 32);

		let y = if m <= 2 {
			y + 1
		} else {
			y
		};

		let y = if y > i16::MAX as i64 {
			Year::MAX
		} else if y < i16::MIN as i64 {
			Year::MIN
		} else {
			Year(y as i16)
		};

		unsafe { Self {
			year: y,
			month: Month::new_unchecked(m),
			day: Day::new_unchecked(d)
		}}
	}

	#[inline]
	/// Convert a [`Date`] to a UNIX timestamp
	///
	/// This converts a calendar day into a UNIX timestamp, assuming UTC.
	///
	/// It is the reverse of [`Date::from_unix`].
	///
	/// A negative `year` represents BCE years, e.g, `-1` is `1 BCE`.
	///
	/// Values before the `UNIX_EPOCH` (before `January 1st, 1970`) will return negative values.
	///
	/// ```rust
	/// # use nichi::*;
	/// let date = Date::new(2023, 10, 20);
	/// assert_eq!(date.as_unix(), 1697760000);
	///
	/// assert_eq!(date, Date::from_unix(date.as_unix()));
	/// ```
	///
	/// ## Algorithm
	/// <https://howardhinnant.github.io/date_algorithms.html#days_from_civil>
	pub const fn as_unix(self) -> i128 {
		let (year, month, day) = self.inner();

		let year  = year as i64;
		let month = month as u64;
		let day   = day as u64;

		let year = if month <= 2 {
			year - 1
		} else {
			year
		};

		let era: i64 = if year >= 0 { year } else { year - 399 } / 400;
		let yoe: u64 = (year - era * 400) as u64;
		let doy: u64 = (153 * if month > 2 { month - 3 } else { month + 9 } + 2) / 5 + day - 1;
		let doe: u64 = yoe * 365 + yoe/4 - yoe/100 + doy;

		((era as i128) * 146097 + (doe as i128) - 719468) * 86400
	}
}

//---------------------------------------------------------------------------------------------------- Impl