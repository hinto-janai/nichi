//---------------------------------------------------------------------------------------------------- Use
use crate::macros::{
	impl_u8_enum,
};

//---------------------------------------------------------------------------------------------------- Month
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Eq,Ord,Hash)]
/// Month in a year
#[allow(missing_docs)]
pub enum Month {
	#[default]
	January   = 1,
	February  = 2,
	March     = 3,
	April     = 4,
	May       = 5,
	June      = 6,
	July      = 7,
	August    = 8,
	September = 9,
	October   = 10,
	November  = 11,
	December  = 12,
}

//---------------------------------------------------------------------------------------------------- Impl
impl Month {
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::FIRST, Month::January);
	/// ```
	pub const FIRST: Month = Month::January;

	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::LAST, Month::December);
	/// ```
	pub const LAST: Month = Month::December;

	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::ALL[0],  Month::January);
	/// assert_eq!(Month::ALL[1],  Month::February);
	/// assert_eq!(Month::ALL[2],  Month::March);
	/// assert_eq!(Month::ALL[3],  Month::April);
	/// assert_eq!(Month::ALL[4],  Month::May);
	/// assert_eq!(Month::ALL[5],  Month::June);
	/// assert_eq!(Month::ALL[6],  Month::July);
	/// assert_eq!(Month::ALL[7],  Month::August);
	/// assert_eq!(Month::ALL[8],  Month::September);
	/// assert_eq!(Month::ALL[9],  Month::October);
	/// assert_eq!(Month::ALL[10], Month::November);
	/// assert_eq!(Month::ALL[11], Month::December);
	/// ```
	pub const ALL: [Month; 12] = [
		Self::January,
		Self::February,
		Self::March,
		Self::April,
		Self::May,
		Self::June,
		Self::July,
		Self::August,
		Self::September,
		Self::October,
		Self::November,
		Self::December,
	];

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::new(1),  Month::January);
	/// assert_eq!(Month::new(2),  Month::February);
	/// assert_eq!(Month::new(3),  Month::March);
	/// assert_eq!(Month::new(4),  Month::April);
	/// assert_eq!(Month::new(5),  Month::May);
	/// assert_eq!(Month::new(6),  Month::June);
	/// assert_eq!(Month::new(7),  Month::July);
	/// assert_eq!(Month::new(8),  Month::August);
	/// assert_eq!(Month::new(9),  Month::September);
	/// assert_eq!(Month::new(10), Month::October);
	/// assert_eq!(Month::new(11), Month::November);
	/// assert_eq!(Month::new(12), Month::December);
	/// ```
	///
	/// ```rust,should_panic
	/// # use nichi::*;
	/// Month::new(0);
	/// ```
	/// ```rust,should_panic
	/// # use nichi::*;
	/// Month::new(13);
	/// ```
	pub const fn new(month: u8) -> Self {
		match month {
			1  => Self::January,
			2  => Self::February,
			3  => Self::March,
			4  => Self::April,
			5  => Self::May,
			6  => Self::June,
			7  => Self::July,
			8  => Self::August,
			9  => Self::September,
			10 => Self::October,
			11 => Self::November,
			12 => Self::December,
			_  => panic!("month is not in-between 1..=12"),
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::new_saturating(1),  Month::January);
	/// assert_eq!(Month::new_saturating(2),  Month::February);
	/// assert_eq!(Month::new_saturating(3),  Month::March);
	/// assert_eq!(Month::new_saturating(4),  Month::April);
	/// assert_eq!(Month::new_saturating(5),  Month::May);
	/// assert_eq!(Month::new_saturating(6),  Month::June);
	/// assert_eq!(Month::new_saturating(7),  Month::July);
	/// assert_eq!(Month::new_saturating(8),  Month::August);
	/// assert_eq!(Month::new_saturating(9),  Month::September);
	/// assert_eq!(Month::new_saturating(10), Month::October);
	/// assert_eq!(Month::new_saturating(11), Month::November);
	/// assert_eq!(Month::new_saturating(12), Month::December);
	/// ```
	///
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::new_saturating(0), Month::January);
	/// ```
	///
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::new_saturating(13), Month::December);
	/// assert_eq!(Month::new_saturating(14), Month::December);
	/// ```
	pub const fn new_saturating(month: u8) -> Self {
		match month {
			0|1 => Self::January,
			2   => Self::February,
			3   => Self::March,
			4   => Self::April,
			5   => Self::May,
			6   => Self::June,
			7   => Self::July,
			8   => Self::August,
			9   => Self::September,
			10  => Self::October,
			11  => Self::November,
			_   => Self::December,
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// // Wraps to December
	/// assert_eq!(Month::new_wrapping(0),  Month::December);
	///
	/// assert_eq!(Month::new_wrapping(1),  Month::January);
	/// assert_eq!(Month::new_wrapping(2),  Month::February);
	/// assert_eq!(Month::new_wrapping(3),  Month::March);
	/// assert_eq!(Month::new_wrapping(4),  Month::April);
	/// assert_eq!(Month::new_wrapping(5),  Month::May);
	/// assert_eq!(Month::new_wrapping(6),  Month::June);
	/// assert_eq!(Month::new_wrapping(7),  Month::July);
	/// assert_eq!(Month::new_wrapping(8),  Month::August);
	/// assert_eq!(Month::new_wrapping(9),  Month::September);
	/// assert_eq!(Month::new_wrapping(10), Month::October);
	/// assert_eq!(Month::new_wrapping(11), Month::November);
	/// assert_eq!(Month::new_wrapping(12), Month::December);
	///
	/// // Wraps to January, February, etc
	/// assert_eq!(Month::new_wrapping(13), Month::January);
	/// assert_eq!(Month::new_wrapping(14), Month::February);
	/// assert_eq!(Month::new_wrapping(15), Month::March);
	/// assert_eq!(Month::new_wrapping(16), Month::April);
	/// assert_eq!(Month::new_wrapping(17), Month::May);
	/// assert_eq!(Month::new_wrapping(18), Month::June);
	/// assert_eq!(Month::new_wrapping(19), Month::July);
	/// assert_eq!(Month::new_wrapping(20), Month::August);
	/// assert_eq!(Month::new_wrapping(21), Month::September);
	/// assert_eq!(Month::new_wrapping(22), Month::October);
	/// assert_eq!(Month::new_wrapping(23), Month::November);
	/// assert_eq!(Month::new_wrapping(24), Month::December);
	/// ```
	pub const fn new_wrapping(month: u8) -> Self {
		match month % 12 {
			1  => Self::January,
			2  => Self::February,
			3  => Self::March,
			4  => Self::April,
			5  => Self::May,
			6  => Self::June,
			7  => Self::July,
			8  => Self::August,
			9  => Self::September,
			10 => Self::October,
			11 => Self::November,
			_  => Self::December,
		}
	}

	impl_u8_enum!();

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::January.as_str(),   "January");
	/// assert_eq!(Month::February.as_str(),  "February");
	/// assert_eq!(Month::March.as_str(),     "March");
	/// assert_eq!(Month::April.as_str(),     "April");
	/// assert_eq!(Month::May.as_str(),       "May");
	/// assert_eq!(Month::June.as_str(),      "June");
	/// assert_eq!(Month::July.as_str(),      "July");
	/// assert_eq!(Month::August.as_str(),    "August");
	/// assert_eq!(Month::September.as_str(), "September");
	/// assert_eq!(Month::October.as_str(),   "October");
	/// assert_eq!(Month::November.as_str(),  "November");
	/// assert_eq!(Month::December.as_str(),  "December");
	/// ```
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::January   => "January",
			Self::February  => "February",
			Self::March     => "March",
			Self::April     => "April",
			Self::May       => "May",
			Self::June      => "June",
			Self::July      => "July",
			Self::August    => "August",
			Self::September => "September",
			Self::October   => "October",
			Self::November  => "November",
			Self::December  => "December",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::January.as_str_lower(),   "january");
	/// assert_eq!(Month::February.as_str_lower(),  "february");
	/// assert_eq!(Month::March.as_str_lower(),     "march");
	/// assert_eq!(Month::April.as_str_lower(),     "april");
	/// assert_eq!(Month::May.as_str_lower(),       "may");
	/// assert_eq!(Month::June.as_str_lower(),      "june");
	/// assert_eq!(Month::July.as_str_lower(),      "july");
	/// assert_eq!(Month::August.as_str_lower(),    "august");
	/// assert_eq!(Month::September.as_str_lower(), "september");
	/// assert_eq!(Month::October.as_str_lower(),   "october");
	/// assert_eq!(Month::November.as_str_lower(),  "november");
	/// assert_eq!(Month::December.as_str_lower(),  "december");
	/// ```
	pub const fn as_str_lower(self) -> &'static str {
		match self {
			Self::January   => "january",
			Self::February  => "february",
			Self::March     => "march",
			Self::April     => "april",
			Self::May       => "may",
			Self::June      => "june",
			Self::July      => "july",
			Self::August    => "august",
			Self::September => "september",
			Self::October   => "october",
			Self::November  => "november",
			Self::December  => "december",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::January.as_str_upper(),   "JANUARY");
	/// assert_eq!(Month::February.as_str_upper(),  "FEBRUARY");
	/// assert_eq!(Month::March.as_str_upper(),     "MARCH");
	/// assert_eq!(Month::April.as_str_upper(),     "APRIL");
	/// assert_eq!(Month::May.as_str_upper(),       "MAY");
	/// assert_eq!(Month::June.as_str_upper(),      "JUNE");
	/// assert_eq!(Month::July.as_str_upper(),      "JULY");
	/// assert_eq!(Month::August.as_str_upper(),    "AUGUST");
	/// assert_eq!(Month::September.as_str_upper(), "SEPTEMBER");
	/// assert_eq!(Month::October.as_str_upper(),   "OCTOBER");
	/// assert_eq!(Month::November.as_str_upper(),  "NOVEMBER");
	/// assert_eq!(Month::December.as_str_upper(),  "DECEMBER");
	/// ```
	pub const fn as_str_upper(self) -> &'static str {
		match self {
			Self::January   => "JANUARY",
			Self::February  => "FEBRUARY",
			Self::March     => "MARCH",
			Self::April     => "APRIL",
			Self::May       => "MAY",
			Self::June      => "JUNE",
			Self::July      => "JULY",
			Self::August    => "AUGUST",
			Self::September => "SEPTEMBER",
			Self::October   => "OCTOBER",
			Self::November  => "NOVEMBER",
			Self::December  => "DECEMBER",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::January.as_str_short(),   "Jan");
	/// assert_eq!(Month::February.as_str_short(),  "Feb");
	/// assert_eq!(Month::March.as_str_short(),     "Mar");
	/// assert_eq!(Month::April.as_str_short(),     "Apr");
	/// assert_eq!(Month::May.as_str_short(),       "May");
	/// assert_eq!(Month::June.as_str_short(),      "Jun");
	/// assert_eq!(Month::July.as_str_short(),      "Jul");
	/// assert_eq!(Month::August.as_str_short(),    "Aug");
	/// assert_eq!(Month::September.as_str_short(), "Sep");
	/// assert_eq!(Month::October.as_str_short(),   "Oct");
	/// assert_eq!(Month::November.as_str_short(),  "Nov");
	/// assert_eq!(Month::December.as_str_short(),  "Dec");
	/// ```
	pub const fn as_str_short(self) -> &'static str {
		match self {
			Self::January   => "Jan",
			Self::February  => "Feb",
			Self::March     => "Mar",
			Self::April     => "Apr",
			Self::May       => "May",
			Self::June      => "Jun",
			Self::July      => "Jul",
			Self::August    => "Aug",
			Self::September => "Sep",
			Self::October   => "Oct",
			Self::November  => "Nov",
			Self::December  => "Dec",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::January.as_str_short_lower(),   "jan");
	/// assert_eq!(Month::February.as_str_short_lower(),  "feb");
	/// assert_eq!(Month::March.as_str_short_lower(),     "mar");
	/// assert_eq!(Month::April.as_str_short_lower(),     "apr");
	/// assert_eq!(Month::May.as_str_short_lower(),       "may");
	/// assert_eq!(Month::June.as_str_short_lower(),      "jun");
	/// assert_eq!(Month::July.as_str_short_lower(),      "jul");
	/// assert_eq!(Month::August.as_str_short_lower(),    "aug");
	/// assert_eq!(Month::September.as_str_short_lower(), "sep");
	/// assert_eq!(Month::October.as_str_short_lower(),   "oct");
	/// assert_eq!(Month::November.as_str_short_lower(),  "nov");
	/// assert_eq!(Month::December.as_str_short_lower(),  "dec");
	/// ```
	pub const fn as_str_short_lower(self) -> &'static str {
		match self {
			Self::January   => "jan",
			Self::February  => "feb",
			Self::March     => "mar",
			Self::April     => "apr",
			Self::May       => "may",
			Self::June      => "jun",
			Self::July      => "jul",
			Self::August    => "aug",
			Self::September => "sep",
			Self::October   => "oct",
			Self::November  => "nov",
			Self::December  => "dec",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::January.as_str_short_upper(),   "JAN");
	/// assert_eq!(Month::February.as_str_short_upper(),  "FEB");
	/// assert_eq!(Month::March.as_str_short_upper(),     "MAR");
	/// assert_eq!(Month::April.as_str_short_upper(),     "APR");
	/// assert_eq!(Month::May.as_str_short_upper(),       "MAY");
	/// assert_eq!(Month::June.as_str_short_upper(),      "JUN");
	/// assert_eq!(Month::July.as_str_short_upper(),      "JUL");
	/// assert_eq!(Month::August.as_str_short_upper(),    "AUG");
	/// assert_eq!(Month::September.as_str_short_upper(), "SEP");
	/// assert_eq!(Month::October.as_str_short_upper(),   "OCT");
	/// assert_eq!(Month::November.as_str_short_upper(),  "NOV");
	/// assert_eq!(Month::December.as_str_short_upper(),  "DEC");
	/// ```
	pub const fn as_str_short_upper(self) -> &'static str {
		match self {
			Self::January   => "JAN",
			Self::February  => "FEB",
			Self::March     => "MAR",
			Self::April     => "APR",
			Self::May       => "MAY",
			Self::June      => "JUN",
			Self::July      => "JUL",
			Self::August    => "AUG",
			Self::September => "SEP",
			Self::October   => "OCT",
			Self::November  => "NOV",
			Self::December  => "DEC",
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
	/// (and all case-combinations).
	///
	/// ## Examples
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Month::from_str("January").unwrap(),   Month::January);
	/// assert_eq!(Month::from_str("january").unwrap(),   Month::January);
	/// assert_eq!(Month::from_str("JANUARY").unwrap(),   Month::January);
	/// assert_eq!(Month::from_str("Jan").unwrap(),       Month::January);
	/// assert_eq!(Month::from_str("jan").unwrap(),       Month::January);
	/// assert_eq!(Month::from_str("JAN").unwrap(),       Month::January);
	/// assert_eq!(Month::from_str("February").unwrap(),  Month::February);
	/// assert_eq!(Month::from_str("february").unwrap(),  Month::February);
	/// assert_eq!(Month::from_str("FEBRUARY").unwrap(),  Month::February);
	/// assert_eq!(Month::from_str("Feb").unwrap(),       Month::February);
	/// assert_eq!(Month::from_str("feb").unwrap(),       Month::February);
	/// assert_eq!(Month::from_str("FEB").unwrap(),       Month::February);
	/// assert_eq!(Month::from_str("March").unwrap(),     Month::March);
	/// assert_eq!(Month::from_str("march").unwrap(),     Month::March);
	/// assert_eq!(Month::from_str("MARCH").unwrap(),     Month::March);
	/// assert_eq!(Month::from_str("Mar").unwrap(),       Month::March);
	/// assert_eq!(Month::from_str("mar").unwrap(),       Month::March);
	/// assert_eq!(Month::from_str("MAR").unwrap(),       Month::March);
	/// assert_eq!(Month::from_str("April").unwrap(),     Month::April);
	/// assert_eq!(Month::from_str("april").unwrap(),     Month::April);
	/// assert_eq!(Month::from_str("APRIL").unwrap(),     Month::April);
	/// assert_eq!(Month::from_str("Apr").unwrap(),       Month::April);
	/// assert_eq!(Month::from_str("apr").unwrap(),       Month::April);
	/// assert_eq!(Month::from_str("APR").unwrap(),       Month::April);
	/// assert_eq!(Month::from_str("May").unwrap(),       Month::May);
	/// assert_eq!(Month::from_str("may").unwrap(),       Month::May);
	/// assert_eq!(Month::from_str("MAY").unwrap(),       Month::May);
	/// assert_eq!(Month::from_str("June").unwrap(),      Month::June);
	/// assert_eq!(Month::from_str("june").unwrap(),      Month::June);
	/// assert_eq!(Month::from_str("JUNE").unwrap(),      Month::June);
	/// assert_eq!(Month::from_str("Jun").unwrap(),       Month::June);
	/// assert_eq!(Month::from_str("jun").unwrap(),       Month::June);
	/// assert_eq!(Month::from_str("JUN").unwrap(),       Month::June);
	/// assert_eq!(Month::from_str("July").unwrap(),      Month::July);
	/// assert_eq!(Month::from_str("july").unwrap(),      Month::July);
	/// assert_eq!(Month::from_str("JULY").unwrap(),      Month::July);
	/// assert_eq!(Month::from_str("Jul").unwrap(),       Month::July);
	/// assert_eq!(Month::from_str("jul").unwrap(),       Month::July);
	/// assert_eq!(Month::from_str("JUL").unwrap(),       Month::July);
	/// assert_eq!(Month::from_str("August").unwrap(),    Month::August);
	/// assert_eq!(Month::from_str("august").unwrap(),    Month::August);
	/// assert_eq!(Month::from_str("AUGUST").unwrap(),    Month::August);
	/// assert_eq!(Month::from_str("Aug").unwrap(),       Month::August);
	/// assert_eq!(Month::from_str("aug").unwrap(),       Month::August);
	/// assert_eq!(Month::from_str("AUG").unwrap(),       Month::August);
	/// assert_eq!(Month::from_str("September").unwrap(), Month::September);
	/// assert_eq!(Month::from_str("september").unwrap(), Month::September);
	/// assert_eq!(Month::from_str("SEPTEMBER").unwrap(), Month::September);
	/// assert_eq!(Month::from_str("Sep").unwrap(),       Month::September);
	/// assert_eq!(Month::from_str("sep").unwrap(),       Month::September);
	/// assert_eq!(Month::from_str("SEP").unwrap(),       Month::September);
	/// assert_eq!(Month::from_str("October").unwrap(),   Month::October);
	/// assert_eq!(Month::from_str("october").unwrap(),   Month::October);
	/// assert_eq!(Month::from_str("OCTOBER").unwrap(),   Month::October);
	/// assert_eq!(Month::from_str("Oct").unwrap(),       Month::October);
	/// assert_eq!(Month::from_str("oct").unwrap(),       Month::October);
	/// assert_eq!(Month::from_str("OCT").unwrap(),       Month::October);
	/// assert_eq!(Month::from_str("November").unwrap(),  Month::November);
	/// assert_eq!(Month::from_str("november").unwrap(),  Month::November);
	/// assert_eq!(Month::from_str("NOVEMBER").unwrap(),  Month::November);
	/// assert_eq!(Month::from_str("Nov").unwrap(),       Month::November);
	/// assert_eq!(Month::from_str("nov").unwrap(),       Month::November);
	/// assert_eq!(Month::from_str("NOV").unwrap(),       Month::November);
	/// assert_eq!(Month::from_str("December").unwrap(),  Month::December);
	/// assert_eq!(Month::from_str("december").unwrap(),  Month::December);
	/// assert_eq!(Month::from_str("DECEMBER").unwrap(),  Month::December);
	/// assert_eq!(Month::from_str("Dec").unwrap(),       Month::December);
	/// assert_eq!(Month::from_str("dec").unwrap(),       Month::December);
	/// assert_eq!(Month::from_str("DEC").unwrap(),       Month::December);
	/// ```
	pub const fn from_str(s: &str) -> Option<Self> {
		let bytes = s.as_bytes();
		let len   = bytes.len();

		if len < 3 || len > 9 {
			return None;
		}

		match bytes {
			b"January"   | b"january"   | b"JANUARY"   | b"Jan" | b"jan" | b"JAN" => Some(Self::January),
			b"February"  | b"february"  | b"FEBRUARY"  | b"Feb" | b"feb" | b"FEB" => Some(Self::February),
			b"March"     | b"march"     | b"MARCH"     | b"Mar" | b"mar" | b"MAR" => Some(Self::March),
			b"April"     | b"april"     | b"APRIL"     | b"Apr" | b"apr" | b"APR" => Some(Self::April),
			b"May"       | b"may"       | b"MAY"                                  => Some(Self::May),
			b"June"      | b"june"      | b"JUNE"      | b"Jun" | b"jun" | b"JUN" => Some(Self::June),
			b"July"      | b"july"      | b"JULY"      | b"Jul" | b"jul" | b"JUL" => Some(Self::July),
			b"August"    | b"august"    | b"AUGUST"    | b"Aug" | b"aug" | b"AUG" => Some(Self::August),
			b"September" | b"september" | b"SEPTEMBER" | b"Sep" | b"sep" | b"SEP" => Some(Self::September),
			b"October"   | b"october"   | b"OCTOBER"   | b"Oct" | b"oct" | b"OCT" => Some(Self::October),
			b"November"  | b"november"  | b"NOVEMBER"  | b"Nov" | b"nov" | b"NOV" => Some(Self::November),
			b"December"  | b"december"  | b"DECEMBER"  | b"Dec" | b"dec" | b"DEC" => Some(Self::December),

			_ => None,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Impl
