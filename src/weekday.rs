//---------------------------------------------------------------------------------------------------- Use
use crate::macros::{
	impl_u8_enum, impl_from_u8_enum, impl_impl_from_u8_enum,
};

//---------------------------------------------------------------------------------------------------- Weekday
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Eq,Ord,Hash)]
/// Day of the week
#[allow(missing_docs)]
pub enum Weekday {
	#[default]
    Sunday    = 1,
    Monday    = 2,
    Tuesday   = 3,
    Wednesday = 4,
    Thursday  = 5,
    Friday    = 6,
    Saturday  = 7,
}

//---------------------------------------------------------------------------------------------------- Impl
impl Weekday {
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::FIRST, Weekday::Sunday);
	/// ```
	pub const FIRST: Weekday = Weekday::Sunday;

	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::LAST, Weekday::Saturday);
	/// ```
	pub const LAST: Weekday = Weekday::Saturday;

	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::ALL[0], Weekday::Sunday);
	/// assert_eq!(Weekday::ALL[1], Weekday::Monday);
	/// assert_eq!(Weekday::ALL[2], Weekday::Tuesday);
	/// assert_eq!(Weekday::ALL[3], Weekday::Wednesday);
	/// assert_eq!(Weekday::ALL[4], Weekday::Thursday);
	/// assert_eq!(Weekday::ALL[5], Weekday::Friday);
	/// assert_eq!(Weekday::ALL[6], Weekday::Saturday);
	/// ```
	pub const ALL: [Weekday; 7] = [
		Self::Sunday,
		Self::Monday,
		Self::Tuesday,
		Self::Wednesday,
		Self::Thursday,
		Self::Friday,
		Self::Saturday,
	];

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::new(1), Weekday::Sunday);
	/// assert_eq!(Weekday::new(2), Weekday::Monday);
	/// assert_eq!(Weekday::new(3), Weekday::Tuesday);
	/// assert_eq!(Weekday::new(4), Weekday::Wednesday);
	/// assert_eq!(Weekday::new(5), Weekday::Thursday);
	/// assert_eq!(Weekday::new(6), Weekday::Friday);
	/// assert_eq!(Weekday::new(7), Weekday::Saturday);
	/// ```
	///
	/// ```rust,should_panic
	/// # use nichi::*;
	/// Weekday::new(0);
	/// ```
	///
	/// ```rust,should_panic
	/// # use nichi::*;
	/// Weekday::new(8);
	/// ```
	pub const fn new(weekday: u8) -> Self {
		assert!(weekday != 0, "weekday must not be 0");
		assert!(weekday < 8, "weekday must not be > 7");
		// SAFETY: repr(u8)
		unsafe { Self::new_unchecked(weekday) }
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// unsafe {
	/// 	assert_eq!(Weekday::new_unchecked(1), Weekday::Sunday);
	/// 	assert_eq!(Weekday::new_unchecked(2), Weekday::Monday);
	/// 	assert_eq!(Weekday::new_unchecked(3), Weekday::Tuesday);
	/// 	assert_eq!(Weekday::new_unchecked(4), Weekday::Wednesday);
	/// 	assert_eq!(Weekday::new_unchecked(5), Weekday::Thursday);
	/// 	assert_eq!(Weekday::new_unchecked(6), Weekday::Friday);
	/// 	assert_eq!(Weekday::new_unchecked(7), Weekday::Saturday);
	/// }
	/// ```
	///
	/// ## Safety
	/// `weekday` must be `1..=7`.
	///
	/// ```rust,should_panic
	/// # use nichi::*;
	/// // ⚠️ Undefined behavior.
	/// // Will panic on debug.
	/// unsafe { Weekday::new_unchecked(8) };
	/// ```
	pub const unsafe fn new_unchecked(weekday: u8) -> Self {
		debug_assert!(weekday != 0, "weekday must not be 0");
		debug_assert!(weekday < 8, "weekday must not be > 7");
		// SAFETY: repr(u8)
		std::mem::transmute(weekday)
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::Sunday.inner(),    1);
	/// assert_eq!(Weekday::Monday.inner(),    2);
	/// assert_eq!(Weekday::Tuesday.inner(),   3);
	/// assert_eq!(Weekday::Wednesday.inner(), 4);
	/// assert_eq!(Weekday::Thursday.inner(),  5);
	/// assert_eq!(Weekday::Friday.inner(),    6);
	/// assert_eq!(Weekday::Saturday.inner(),  7);
	/// ```
	pub const fn inner(self) -> u8 {
		// SAFETY: repr(u8)
		unsafe { std::mem::transmute(self) }
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::new_saturating(1), Weekday::Sunday);
	/// assert_eq!(Weekday::new_saturating(2), Weekday::Monday);
	/// assert_eq!(Weekday::new_saturating(3), Weekday::Tuesday);
	/// assert_eq!(Weekday::new_saturating(4), Weekday::Wednesday);
	/// assert_eq!(Weekday::new_saturating(5), Weekday::Thursday);
	/// assert_eq!(Weekday::new_saturating(6), Weekday::Friday);
	/// assert_eq!(Weekday::new_saturating(7), Weekday::Saturday);
	/// ```
	///
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::new_saturating(0), Weekday::Sunday);
	/// ```
	///
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::new_saturating(7), Weekday::Saturday);
	/// assert_eq!(Weekday::new_saturating(8), Weekday::Saturday);
	/// assert_eq!(Weekday::new_saturating(9), Weekday::Saturday);
	/// ```
	pub const fn new_saturating(weekday: u8) -> Self {
		if weekday == 0 {
			Self::FIRST
		} else if weekday < 8 {
			// SAFETY: repr(u8)
			unsafe { Self::new_unchecked(weekday) }
		} else {
			Self::LAST
		}
	}

	#[inline(always)]
	/// ```rust
	/// # use nichi::*;
	/// // Wraps to Saturday.
	/// assert_eq!(Weekday::new_wrapping(0), Weekday::Saturday);
	///
	/// assert_eq!(Weekday::new_wrapping(1), Weekday::Sunday);
	/// assert_eq!(Weekday::new_wrapping(2), Weekday::Monday);
	/// assert_eq!(Weekday::new_wrapping(3), Weekday::Tuesday);
	/// assert_eq!(Weekday::new_wrapping(4), Weekday::Wednesday);
	/// assert_eq!(Weekday::new_wrapping(5), Weekday::Thursday);
	/// assert_eq!(Weekday::new_wrapping(6), Weekday::Friday);
	/// assert_eq!(Weekday::new_wrapping(7), Weekday::Saturday);
	///
	/// // Wraps to Sunday, Monday, etc
	/// assert_eq!(Weekday::new_wrapping(8),  Weekday::Sunday);
	/// assert_eq!(Weekday::new_wrapping(9),  Weekday::Monday);
	/// assert_eq!(Weekday::new_wrapping(10), Weekday::Tuesday);
	/// assert_eq!(Weekday::new_wrapping(11), Weekday::Wednesday);
	/// assert_eq!(Weekday::new_wrapping(12), Weekday::Thursday);
	/// assert_eq!(Weekday::new_wrapping(13), Weekday::Friday);
	/// assert_eq!(Weekday::new_wrapping(14), Weekday::Saturday);
	/// ```
	pub const fn new_wrapping(weekday: u8) -> Self {
		let weekday = weekday % 7;
		if weekday == 0 {
			Self::LAST
		} else {
			// SAFETY: repr(u8)
			unsafe { Self::new_unchecked(weekday) }
		}
	}

	impl_u8_enum!();

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::Sunday.as_str(),    "Sunday");
	/// assert_eq!(Weekday::Monday.as_str(),    "Monday");
	/// assert_eq!(Weekday::Tuesday.as_str(),   "Tuesday");
	/// assert_eq!(Weekday::Wednesday.as_str(), "Wednesday");
	/// assert_eq!(Weekday::Thursday.as_str(),  "Thursday");
	/// assert_eq!(Weekday::Friday.as_str(),    "Friday");
	/// assert_eq!(Weekday::Saturday.as_str(),  "Saturday");
	/// ```
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::Sunday    => "Sunday",
			Self::Monday    => "Monday",
			Self::Tuesday   => "Tuesday",
			Self::Wednesday => "Wednesday",
			Self::Thursday  => "Thursday",
			Self::Friday    => "Friday",
			Self::Saturday  => "Saturday",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::Sunday.as_str_lower(),    "sunday");
	/// assert_eq!(Weekday::Monday.as_str_lower(),    "monday");
	/// assert_eq!(Weekday::Tuesday.as_str_lower(),   "tuesday");
	/// assert_eq!(Weekday::Wednesday.as_str_lower(), "wednesday");
	/// assert_eq!(Weekday::Thursday.as_str_lower(),  "thursday");
	/// assert_eq!(Weekday::Friday.as_str_lower(),    "friday");
	/// assert_eq!(Weekday::Saturday.as_str_lower(),  "saturday");
	/// ```
	pub const fn as_str_lower(self) -> &'static str {
		match self {
			Self::Sunday    => "sunday",
			Self::Monday    => "monday",
			Self::Tuesday   => "tuesday",
			Self::Wednesday => "wednesday",
			Self::Thursday  => "thursday",
			Self::Friday    => "friday",
			Self::Saturday  => "saturday",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::Sunday.as_str_upper(),    "SUNDAY");
	/// assert_eq!(Weekday::Monday.as_str_upper(),    "MONDAY");
	/// assert_eq!(Weekday::Tuesday.as_str_upper(),   "TUESDAY");
	/// assert_eq!(Weekday::Wednesday.as_str_upper(), "WEDNESDAY");
	/// assert_eq!(Weekday::Thursday.as_str_upper(),  "THURSDAY");
	/// assert_eq!(Weekday::Friday.as_str_upper(),    "FRIDAY");
	/// assert_eq!(Weekday::Saturday.as_str_upper(),  "SATURDAY");
	/// ```
	pub const fn as_str_upper(self) -> &'static str {
		match self {
			Self::Sunday    => "SUNDAY",
			Self::Monday    => "MONDAY",
			Self::Tuesday   => "TUESDAY",
			Self::Wednesday => "WEDNESDAY",
			Self::Thursday  => "THURSDAY",
			Self::Friday    => "FRIDAY",
			Self::Saturday  => "SATURDAY",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::Sunday.as_str_short(),    "Sun");
	/// assert_eq!(Weekday::Monday.as_str_short(),    "Mon");
	/// assert_eq!(Weekday::Tuesday.as_str_short(),   "Tue");
	/// assert_eq!(Weekday::Wednesday.as_str_short(), "Wed");
	/// assert_eq!(Weekday::Thursday.as_str_short(),  "Thu");
	/// assert_eq!(Weekday::Friday.as_str_short(),    "Fri");
	/// assert_eq!(Weekday::Saturday.as_str_short(),  "Sat");
	/// ```
	pub const fn as_str_short(self) -> &'static str {
		match self {
			Self::Sunday    => "Sun",
			Self::Monday    => "Mon",
			Self::Tuesday   => "Tue",
			Self::Wednesday => "Wed",
			Self::Thursday  => "Thu",
			Self::Friday    => "Fri",
			Self::Saturday  => "Sat",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::Sunday.as_str_short_lower(),    "sun");
	/// assert_eq!(Weekday::Monday.as_str_short_lower(),    "mon");
	/// assert_eq!(Weekday::Tuesday.as_str_short_lower(),   "tue");
	/// assert_eq!(Weekday::Wednesday.as_str_short_lower(), "wed");
	/// assert_eq!(Weekday::Thursday.as_str_short_lower(),  "thu");
	/// assert_eq!(Weekday::Friday.as_str_short_lower(),    "fri");
	/// assert_eq!(Weekday::Saturday.as_str_short_lower(),  "sat");
	/// ```
	pub const fn as_str_short_lower(self) -> &'static str {
		match self {
			Self::Sunday    => "sun",
			Self::Monday    => "mon",
			Self::Tuesday   => "tue",
			Self::Wednesday => "wed",
			Self::Thursday  => "thu",
			Self::Friday    => "fri",
			Self::Saturday  => "sat",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::Sunday.as_str_short_upper(),    "SUN");
	/// assert_eq!(Weekday::Monday.as_str_short_upper(),    "MON");
	/// assert_eq!(Weekday::Tuesday.as_str_short_upper(),   "TUE");
	/// assert_eq!(Weekday::Wednesday.as_str_short_upper(), "WED");
	/// assert_eq!(Weekday::Thursday.as_str_short_upper(),  "THU");
	/// assert_eq!(Weekday::Friday.as_str_short_upper(),    "FRI");
	/// assert_eq!(Weekday::Saturday.as_str_short_upper(),  "SAT");
	/// ```
	pub const fn as_str_short_upper(self) -> &'static str {
		match self {
			Self::Sunday    => "SUN",
			Self::Monday    => "MON",
			Self::Tuesday   => "TUE",
			Self::Wednesday => "WED",
			Self::Thursday  => "THU",
			Self::Friday    => "FRI",
			Self::Saturday  => "SAT",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::Sunday.as_str_jp(),    "日曜日");
	/// assert_eq!(Weekday::Monday.as_str_jp(),    "月曜日");
	/// assert_eq!(Weekday::Tuesday.as_str_jp(),   "火曜日");
	/// assert_eq!(Weekday::Wednesday.as_str_jp(), "水曜日");
	/// assert_eq!(Weekday::Thursday.as_str_jp(),  "木曜日");
	/// assert_eq!(Weekday::Friday.as_str_jp(),    "金曜日");
	/// assert_eq!(Weekday::Saturday.as_str_jp(),  "土曜日");
	/// ```
	pub const fn as_str_jp(self) -> &'static str {
		match self {
			Self::Sunday    => "日曜日",
			Self::Monday    => "月曜日",
			Self::Tuesday   => "火曜日",
			Self::Wednesday => "水曜日",
			Self::Thursday  => "木曜日",
			Self::Friday    => "金曜日",
			Self::Saturday  => "土曜日",
		}
	}

	#[inline]
	/// Create a [`Weekday`] by parsing a [`&str`]
	///
	/// A valid input string can either be the first 3 letters of the day (returned from [`Weekday::as_str_short`])
	/// or the full weekday (returned from [`Weekday::as_str`]).
	///
	/// These cases are covered:
	/// - `lowercase`
	/// - `UPPERCASE`
	///
	/// For example:
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::from_str("SUN").unwrap(),    Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("Sun").unwrap(),    Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("sun").unwrap(),    Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("SUNDAY").unwrap(), Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("Sunday").unwrap(), Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("sunday").unwrap(), Weekday::Sunday);
	/// ```
	///
	/// If:
	/// - `s.len() < 3`
	/// - `s.len() > 9`
	/// - The string could not be parsed
	/// then this function will return `None`.
	///
	/// ## Parsing Exceptions
	/// [`Weekday::Tuesday`] can be parsed from
	/// - `tue`
	/// - `tues`
	///
	/// [`Weekday::Thursday`] can be parsed from:
	/// - `thu`
	/// - `thur`
	/// - `thurs`
	///
	/// (and all case-combinations).
	///
	/// ## Examples
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Weekday::from_str("SUN").unwrap(),    Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("Sun").unwrap(),    Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("sun").unwrap(),    Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("SUNDAY").unwrap(), Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("Sunday").unwrap(), Weekday::Sunday);
	/// assert_eq!(Weekday::from_str("sunday").unwrap(), Weekday::Sunday);
	///
	/// assert_eq!(Weekday::from_str("MON").unwrap(),    Weekday::Monday);
	/// assert_eq!(Weekday::from_str("Mon").unwrap(),    Weekday::Monday);
	/// assert_eq!(Weekday::from_str("mon").unwrap(),    Weekday::Monday);
	/// assert_eq!(Weekday::from_str("MONDAY").unwrap(), Weekday::Monday);
	/// assert_eq!(Weekday::from_str("Monday").unwrap(), Weekday::Monday);
	/// assert_eq!(Weekday::from_str("monday").unwrap(), Weekday::Monday);
	///
	/// assert_eq!(Weekday::from_str("TUES").unwrap(),    Weekday::Tuesday);
	/// assert_eq!(Weekday::from_str("Tues").unwrap(),    Weekday::Tuesday);
	/// assert_eq!(Weekday::from_str("tues").unwrap(),    Weekday::Tuesday);
	/// assert_eq!(Weekday::from_str("TUE").unwrap(),     Weekday::Tuesday);
	/// assert_eq!(Weekday::from_str("Tue").unwrap(),     Weekday::Tuesday);
	/// assert_eq!(Weekday::from_str("tue").unwrap(),     Weekday::Tuesday);
	/// assert_eq!(Weekday::from_str("TUESDAY").unwrap(), Weekday::Tuesday);
	/// assert_eq!(Weekday::from_str("Tuesday").unwrap(), Weekday::Tuesday);
	/// assert_eq!(Weekday::from_str("tuesday").unwrap(), Weekday::Tuesday);
	///
	/// assert_eq!(Weekday::from_str("WED").unwrap(),       Weekday::Wednesday);
	/// assert_eq!(Weekday::from_str("Wed").unwrap(),       Weekday::Wednesday);
	/// assert_eq!(Weekday::from_str("wed").unwrap(),       Weekday::Wednesday);
	/// assert_eq!(Weekday::from_str("WEDNESDAY").unwrap(), Weekday::Wednesday);
	/// assert_eq!(Weekday::from_str("Wednesday").unwrap(), Weekday::Wednesday);
	/// assert_eq!(Weekday::from_str("wednesday").unwrap(), Weekday::Wednesday);
	///
	/// assert_eq!(Weekday::from_str("THURS").unwrap(),    Weekday::Thursday);
	/// assert_eq!(Weekday::from_str("Thurs").unwrap(),    Weekday::Thursday);
	/// assert_eq!(Weekday::from_str("thurs").unwrap(),    Weekday::Thursday);
	/// assert_eq!(Weekday::from_str("THUR").unwrap(),     Weekday::Thursday);
	/// assert_eq!(Weekday::from_str("Thur").unwrap(),     Weekday::Thursday);
	/// assert_eq!(Weekday::from_str("thur").unwrap(),     Weekday::Thursday);
	/// assert_eq!(Weekday::from_str("THURSDAY").unwrap(), Weekday::Thursday);
	/// assert_eq!(Weekday::from_str("Thursday").unwrap(), Weekday::Thursday);
	/// assert_eq!(Weekday::from_str("thursday").unwrap(), Weekday::Thursday);
	///
	/// assert_eq!(Weekday::from_str("FRI").unwrap(),    Weekday::Friday);
	/// assert_eq!(Weekday::from_str("Fri").unwrap(),    Weekday::Friday);
	/// assert_eq!(Weekday::from_str("fri").unwrap(),    Weekday::Friday);
	/// assert_eq!(Weekday::from_str("FRIDAY").unwrap(), Weekday::Friday);
	/// assert_eq!(Weekday::from_str("Friday").unwrap(), Weekday::Friday);
	/// assert_eq!(Weekday::from_str("friday").unwrap(), Weekday::Friday);
	///
	/// assert_eq!(Weekday::from_str("SAT").unwrap(),      Weekday::Saturday);
	/// assert_eq!(Weekday::from_str("Sat").unwrap(),      Weekday::Saturday);
	/// assert_eq!(Weekday::from_str("sat").unwrap(),      Weekday::Saturday);
	/// assert_eq!(Weekday::from_str("SATURDAY").unwrap(), Weekday::Saturday);
	/// assert_eq!(Weekday::from_str("Saturday").unwrap(), Weekday::Saturday);
	/// assert_eq!(Weekday::from_str("saturday").unwrap(), Weekday::Saturday);
	/// ```
	pub const fn from_str(s: &str) -> Option<Self> {
		Self::from_bytes(s.as_bytes())
	}

	#[inline]
	/// Same as [`Self::from_str`] but with bytes
	///
	/// ## Safety
	/// `bytes` must be valid UTF-8.
	pub const fn from_bytes(bytes: &[u8]) -> Option<Self> {
		let len   = bytes.len();

		if len < 3 || len > 9 {
			return None;
		}

		match bytes {
			b"Sun" | b"Sunday"   | b"sun"  | b"sunday"   | b"SUN" | b"SUNDAY" => Some(Self::Sunday),
			b"Mon" | b"Monday"   | b"mon"  | b"monday"   | b"MON" | b"MONDAY" => Some(Self::Monday),
			b"Tue" | b"Tuesday"  | b"tue"  | b"tuesday"  | b"TUE" | b"TUESDAY" |  b"Tues" | b"tues" | b"TUES" => Some(Self::Tuesday),
			b"Wed" | b"Wednesday"| b"wed"  | b"wednesday"| b"WED" | b"WEDNESDAY" => Some(Self::Wednesday),
			b"Thu" | b"Thursday" | b"thu"  | b"thursday" | b"THU" | b"THURSDAY" | b"thur" | b"Thur" | b"THUR" | b"thurs" | b"Thurs" | b"THURS" => Some(Self::Thursday),
			b"Fri" | b"Friday"   | b"fri"  | b"friday"   | b"FRI" | b"FRIDAY" => Some(Self::Friday),
			b"Sat" | b"Saturday" | b"sat"  | b"saturday" | b"SAT" | b"SATURDAY"  => Some(Self::Saturday),
			_ => None,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Trait
impl_from_u8_enum!(Weekday);