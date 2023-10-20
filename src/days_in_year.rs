//---------------------------------------------------------------------------------------------------- Use
use derive_more::Display;
use crate::macros::impl_traits;

//---------------------------------------------------------------------------------------------------- DaysInYear
///
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Eq,Ord,Hash)]
#[derive(Display)]
/// Days in a year accounting for leap years
pub enum DaysInYear {
	#[default]
	/// `365` days
	ThreeSixFive,
	/// `366` days (leap year)
	ThreeSixSix,
}

impl_traits!{ DaysInYear => u16 |
	u8,u16,u32,u64,u128,usize |
	i8,i16,i32,i64,i128,isize
}

//---------------------------------------------------------------------------------------------------- Impl
impl DaysInYear {
	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInYear::ThreeSixFive.inner(), 365);
	/// assert_eq!(DaysInYear::ThreeSixSix.inner(), 366);
	/// ```
	pub const fn inner(self) -> u16 {
		match self {
			Self::ThreeSixFive => 365,
			Self::ThreeSixSix => 366,
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInYear::ThreeSixFive.is_365(), true);
	/// assert_eq!(DaysInYear::ThreeSixSix.is_365(),  false);
	/// ```
	pub const fn is_365(self) -> bool {
		match self {
			Self::ThreeSixFive => true,
			Self::ThreeSixSix => false,
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInYear::ThreeSixFive.is_366(), false);
	/// assert_eq!(DaysInYear::ThreeSixSix.is_366(),  true);
	/// ```
	pub const fn is_366(self) -> bool {
		match self {
			Self::ThreeSixFive => false,
			Self::ThreeSixSix => true,
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInYear::ThreeSixFive.as_str(), "ThreeSixFive");
	/// assert_eq!(DaysInYear::ThreeSixSix.as_str(),  "ThreeSixSix");
	/// ```
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::ThreeSixFive => "ThreeSixFive",
			Self::ThreeSixSix => "ThreeSixSix",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInYear::ThreeSixFive.as_str_num(), "365");
	/// assert_eq!(DaysInYear::ThreeSixSix.as_str_num(),  "366");
	/// ```
	pub const fn as_str_num(self) -> &'static str {
		match self {
			Self::ThreeSixFive => "365",
			Self::ThreeSixSix => "366",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInYear::ThreeSixFive.as_str_lower(), "threesixfive");
	/// assert_eq!(DaysInYear::ThreeSixSix.as_str_lower(),  "threesixsix");
	/// ```
	pub const fn as_str_lower(self) -> &'static str {
		match self {
			Self::ThreeSixFive => "threesixfive",
			Self::ThreeSixSix => "threesixsix",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInYear::ThreeSixFive.as_str_upper(), "THREESIXFIVE");
	/// assert_eq!(DaysInYear::ThreeSixSix.as_str_upper(),  "THREESIXSIX");
	/// ```
	pub const fn as_str_upper(self) -> &'static str {
		match self {
			Self::ThreeSixFive => "THREESIXFIVE",
			Self::ThreeSixSix => "THREESIXSIX",
		}
	}
}

//---------------------------------------------------------------------------------------------------- Trait