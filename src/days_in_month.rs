//---------------------------------------------------------------------------------------------------- Use
use derive_more::Display;
use crate::macros::impl_traits;
use crate::day::Day;

//---------------------------------------------------------------------------------------------------- DaysInMonth
/// Days in a month
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Eq,Ord,Hash)]
#[derive(Display)]
pub enum DaysInMonth {
	#[default]
	/// `31st`
	ThirtyOne = 31,
	/// `30th`
	Thirty = 30,
	/// `29th` (February leap year)
	TwentyNine = 29,
	/// `28th` (February non-leap year)
	TwentyEight = 28,
}

impl_traits!{ DaysInMonth => u8 |
	u8,u16,u32,u64,u128,usize |
	i8,i16,i32,i64,i128,isize
}

//---------------------------------------------------------------------------------------------------- Impl
impl DaysInMonth {
	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInMonth::ThirtyOne.inner(),   31);
	/// assert_eq!(DaysInMonth::Thirty.inner(),      30);
	/// assert_eq!(DaysInMonth::TwentyNine.inner(),  29);
	/// assert_eq!(DaysInMonth::TwentyEight.inner(), 28);
	/// ```
	pub const fn inner(self) -> u8 {
		// SAFETY: repr(u8)
		unsafe { std::mem::transmute(self) }
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInMonth::new(31), DaysInMonth::ThirtyOne);
	/// assert_eq!(DaysInMonth::new(30), DaysInMonth::Thirty);
	/// assert_eq!(DaysInMonth::new(29), DaysInMonth::TwentyNine);
	/// assert_eq!(DaysInMonth::new(28), DaysInMonth::TwentyEight);
	/// ```
	/// ```rust,should_panic
	/// # use nichi::*;
	/// DaysInMonth::new(27);
	/// ```
	/// ```rust,should_panic
	/// # use nichi::*;
	/// DaysInMonth::new(32);
	/// ```
	pub const fn new(day: u8) -> Self {
		assert!(day > 27, "day must not be < 28");
		assert!(day < 32, "day must not be > 31");
		// SAFETY: repr(u8)
		unsafe { Self::new_unchecked(day) }
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// unsafe {
	/// 	assert_eq!(DaysInMonth::new_unchecked(31), DaysInMonth::ThirtyOne);
	/// 	assert_eq!(DaysInMonth::new_unchecked(30), DaysInMonth::Thirty);
	/// 	assert_eq!(DaysInMonth::new_unchecked(29), DaysInMonth::TwentyNine);
	/// 	assert_eq!(DaysInMonth::new_unchecked(28), DaysInMonth::TwentyEight);
	/// }
	/// ```
	///
	/// ## Safety
	/// `day` must be `31`, `30`, `29`, or `28`.
	/// ```rust,should_panic
	/// # use nichi::*;
	/// // ⚠️ Undefined behavior.
	/// // Will panic on debug.
	/// unsafe { DaysInMonth::new_unchecked(27) };
	/// ```
	pub const unsafe fn new_unchecked(day: u8) -> Self {
		debug_assert!(day > 27, "day must not be < 28");
		debug_assert!(day < 32, "day must not be > 31");
		// SAFETY: repr(u8)
		std::mem::transmute(day)
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInMonth::ThirtyOne.as_str(),  "ThirtyOne");
	/// assert_eq!(DaysInMonth::Thirty.as_str(),    "Thirty");
	/// assert_eq!(DaysInMonth::TwentyNine.as_str(),  "TwentyNine");
	/// assert_eq!(DaysInMonth::TwentyEight.as_str(), "TwentyEight");
	/// ```
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::ThirtyOne  => "ThirtyOne",
			Self::Thirty    => "Thirty",
			Self::TwentyNine  => "TwentyNine",
			Self::TwentyEight => "TwentyEight",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInMonth::ThirtyOne.as_str_num(),  "31");
	/// assert_eq!(DaysInMonth::Thirty.as_str_num(),    "30");
	/// assert_eq!(DaysInMonth::TwentyNine.as_str_num(),  "29");
	/// assert_eq!(DaysInMonth::TwentyEight.as_str_num(), "28");
	/// ```
	pub const fn as_str_num(self) -> &'static str {
		match self {
			Self::ThirtyOne  => "31",
			Self::Thirty    => "30",
			Self::TwentyNine  => "29",
			Self::TwentyEight => "28",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInMonth::ThirtyOne.as_str_lower(),  "thirtyfirst");
	/// assert_eq!(DaysInMonth::Thirty.as_str_lower(),    "thirtieth");
	/// assert_eq!(DaysInMonth::TwentyNine.as_str_lower(),  "twentyninth");
	/// assert_eq!(DaysInMonth::TwentyEight.as_str_lower(), "twentyeighth");
	/// ```
	pub const fn as_str_lower(self) -> &'static str {
		match self {
			Self::ThirtyOne  => "thirtyfirst",
			Self::Thirty    => "thirtieth",
			Self::TwentyNine  => "twentyninth",
			Self::TwentyEight => "twentyeighth",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInMonth::ThirtyOne.as_str_upper(),  "THIRTYFIRST");
	/// assert_eq!(DaysInMonth::Thirty.as_str_upper(),    "THIRTIETH");
	/// assert_eq!(DaysInMonth::TwentyNine.as_str_upper(),  "TWENTYNINTH");
	/// assert_eq!(DaysInMonth::TwentyEight.as_str_upper(), "TWENTYEIGHTH");
	/// ```
	pub const fn as_str_upper(self) -> &'static str {
		match self {
			Self::ThirtyOne  => "THIRTYFIRST",
			Self::Thirty    => "THIRTIETH",
			Self::TwentyNine  => "TWENTYNINTH",
			Self::TwentyEight => "TWENTYEIGHTH",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(DaysInMonth::ThirtyOne.as_day(),   Day::ThirtyFirst);
	/// assert_eq!(DaysInMonth::Thirty.as_day(),      Day::Thirtieth);
	/// assert_eq!(DaysInMonth::TwentyNine.as_day(),  Day::TwentyNinth);
	/// assert_eq!(DaysInMonth::TwentyEight.as_day(), Day::TwentyEighth);
	/// ```
	pub const fn as_day(self) -> Day {
		// SAFETY: both enums are repr(c)
		unsafe { std::mem::transmute(self) }
	}
}

//---------------------------------------------------------------------------------------------------- Trait