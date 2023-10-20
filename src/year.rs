//---------------------------------------------------------------------------------------------------- Use
use derive_more::{
	Add,
	Display,
	From,
	Into,
	Deref,
	Not,
	Mul,
	Sum,
	DerefMut,
	AddAssign,
	MulAssign,
};
use crate::day::Day;
use crate::month::Month;
use crate::weekday::Weekday;
use crate::days_in_year::DaysInYear;
use crate::days_in_month::DaysInMonth;
use crate::macros::impl_traits;

//---------------------------------------------------------------------------------------------------- Year
/// Any year from `-32,768` to `32,767`
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Eq,Ord,Hash)]
#[derive(
	Add,
	Display,
	From,
	Deref,
	Not,
	Mul,
	Sum,
	DerefMut,
	AddAssign,
	MulAssign,
)]
pub struct Year(pub i16);

impl_traits!{ Year => i16 |
	u8,u16,u32,u64,u128,usize |
	i8,i16,i32,i64,i128,isize
}

//---------------------------------------------------------------------------------------------------- Impl
impl Year {
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Year::MAX, Year(i16::MAX));
	/// ```
	pub const MAX: Year = Year(i16::MAX);
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Year::MIN, Year(i16::MIN));
	/// ```
	pub const MIN: Year = Year(i16::MIN);

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert!(!Year(2019).is_leap());
	/// assert!(Year(2020).is_leap());
	/// assert!(!Year(2023).is_leap());
	/// assert!(Year(2024).is_leap());
	/// ```
	pub const fn is_leap(self) -> bool {
		(self.0 % 4 == 0 && self.0 % 100 != 0) || self.0 % 400 == 0
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// // Non-leap year.
	/// assert_eq!(Year(2019).days_in_year(), 365);
	/// // Leap year.
	/// assert_eq!(Year(2020).days_in_year(), 366);
	/// ```
	pub const fn days_in_year(self) -> DaysInYear {
		if self.is_leap() {
			DaysInYear::ThreeSixSix
		} else {
			DaysInYear::ThreeSixFive
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(
	/// 	Year(2019).days_in_month(Month::January),
	/// 	DaysInMonth::ThirtyOne,
	/// );
	///
	/// assert_eq!(
	/// 	Year(2019).days_in_month(Month::April),
	/// 	DaysInMonth::Thirty,
	/// );
	///
	/// assert_eq!(
	/// 	Year(2019).days_in_month(Month::February),
	/// 	DaysInMonth::TwentyEight,
	/// );
	///
	/// // Leap Year
	/// assert_eq!(
	/// 	Year(2020).days_in_month(Month::February),
	/// 	DaysInMonth::TwentyNine,
	/// );
	/// ```
	pub const fn days_in_month(self, month: Month) -> DaysInMonth {
		use Month as M;
		match month {
			M::January|M::March|M::May|M::July|
			M::August|M::October |M::December => DaysInMonth::ThirtyOne,

			M::April|M::June|M::September|M::November => DaysInMonth::Thirty,

			M::February => if self.is_leap() {
				DaysInMonth::TwentyNine
			} else {
				DaysInMonth::TwentyEight
			}
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Year(2000).inner(), 2000);
	/// ```
	pub const fn inner(self) -> i16 {
		self.0
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Year::from_str("-32768").unwrap(), Year(-32768));
	/// assert_eq!(Year::from_str("0").unwrap(),     Year(0));
	/// assert_eq!(Year::from_str("2000").unwrap(),  Year(2000));
	/// assert_eq!(Year::from_str("32767").unwrap(), Year(32767));
	/// ```
	pub fn from_str(s: &str) -> Option<Self> {
		match s.parse::<i16>() {
			Ok(u) => Some(Year(u)),
			_ => None,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Floats
macro_rules! impl_f {
	($from:ty) => {
		impl From<$from> for Year {
			#[inline]
			fn from(f: $from) -> Self {
				if !f.is_finite() {
					return Self(0);
				}

				if f > i16::MAX as $from {
					return Self::MAX;
				} else if f < i16::MIN as $from {
					return Self::MIN;
				}

				Self(f as i16)
			}
		}
		impl From<&$from> for Year {
			#[inline]
			fn from(f: &$from) -> Self {
				Self::from(*f)
			}
		}
	}
}
impl_f!(f32);
impl_f!(f64);

//---------------------------------------------------------------------------------------------------- uint
macro_rules! impl_u {
	($from:ty) => {
		impl From<$from> for Year {
			#[inline]
			fn from(year: $from) -> Self {
				if year > i16::MAX as $from {
					return Self::MAX;
				}
				Self(year as i16)
			}
		}
		impl From<&$from> for Year {
			#[inline]
			fn from(year: &$from) -> Self {
				Self::from(*year)
			}
		}
	}
}
impl_u!(u8);
impl_u!(u32);
impl_u!(u64);
impl_u!(u128);
impl_u!(usize);

//---------------------------------------------------------------------------------------------------- Int
macro_rules! impl_i {
	($from:ty) => {
		impl From<$from> for Year {
			#[inline]
			fn from(year: $from) -> Self {
				if year > i16::MAX as $from {
					return Self::MAX;
				}
				Self(year as i16)

			}
		}
		impl From<&$from> for Year {
			#[inline]
			fn from(year: &$from) -> Self {
				Self::from(*year)
			}
		}
	}
}
impl_i!(i8);
impl_i!(i32);
impl_i!(i64);
impl_i!(i128);
impl_i!(isize);