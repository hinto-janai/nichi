//---------------------------------------------------------------------------------------------------- Use
use crate::macros::{
	impl_u8_enum,
};

//---------------------------------------------------------------------------------------------------- Day
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Eq,Ord,Hash)]
/// Day of the month
#[allow(missing_docs)]
pub enum Day {
	#[default]
	First         = 1,
	Second        = 2,
	Third         = 3,
	Fourth        = 4,
	Fifth         = 5,
	Sixth         = 6,
	Seventh       = 7,
	Eighth        = 8,
	Ninth         = 9,
	Tenth         = 10,
	Eleventh      = 11,
	Twelfth       = 12,
	Thirteenth    = 13,
	Fourteenth    = 14,
	Fifteenth     = 15,
	Sixteenth     = 16,
	Seventeenth   = 17,
	Eighteenth    = 18,
	Nineteenth    = 19,
	Twentieth     = 20,
	TwentyFirst   = 21,
	TwentySecond  = 22,
	TwentyThird   = 23,
	TwentyFourth  = 24,
	TwentyFifth   = 25,
	TwentySixth   = 26,
	TwentySeventh = 27,
	TwentyEighth  = 28,
	TwentyNinth   = 29,
	Thirtieth     = 30,
	ThirtyFirst   = 31,
}

//---------------------------------------------------------------------------------------------------- Impl
impl Day {
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::FIRST, Day::First);
	/// ```
	pub const FIRST: Day = Day::First;

	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::LAST, Day::ThirtyFirst);
	/// ```
	pub const LAST: Day = Day::ThirtyFirst;

	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::ALL[0],  Day::First);
	/// assert_eq!(Day::ALL[1],  Day::Second);
	/// assert_eq!(Day::ALL[2],  Day::Third);
	/// assert_eq!(Day::ALL[3],  Day::Fourth);
	/// assert_eq!(Day::ALL[4],  Day::Fifth);
	/// assert_eq!(Day::ALL[5],  Day::Sixth);
	/// assert_eq!(Day::ALL[6],  Day::Seventh);
	/// assert_eq!(Day::ALL[7],  Day::Eighth);
	/// assert_eq!(Day::ALL[8],  Day::Ninth);
	/// assert_eq!(Day::ALL[9],  Day::Tenth);
	/// assert_eq!(Day::ALL[10], Day::Eleventh);
	/// assert_eq!(Day::ALL[11], Day::Twelfth);
	/// assert_eq!(Day::ALL[12], Day::Thirteenth);
	/// assert_eq!(Day::ALL[13], Day::Fourteenth);
	/// assert_eq!(Day::ALL[14], Day::Fifteenth);
	/// assert_eq!(Day::ALL[15], Day::Sixteenth);
	/// assert_eq!(Day::ALL[16], Day::Seventeenth);
	/// assert_eq!(Day::ALL[17], Day::Eighteenth);
	/// assert_eq!(Day::ALL[18], Day::Nineteenth);
	/// assert_eq!(Day::ALL[19], Day::Twentieth);
	/// assert_eq!(Day::ALL[20], Day::TwentyFirst);
	/// assert_eq!(Day::ALL[21], Day::TwentySecond);
	/// assert_eq!(Day::ALL[22], Day::TwentyThird);
	/// assert_eq!(Day::ALL[23], Day::TwentyFourth);
	/// assert_eq!(Day::ALL[24], Day::TwentyFifth);
	/// assert_eq!(Day::ALL[25], Day::TwentySixth);
	/// assert_eq!(Day::ALL[26], Day::TwentySeventh);
	/// assert_eq!(Day::ALL[27], Day::TwentyEighth);
	/// assert_eq!(Day::ALL[28], Day::TwentyNinth);
	/// assert_eq!(Day::ALL[29], Day::Thirtieth);
	/// assert_eq!(Day::ALL[30], Day::ThirtyFirst);
	/// ```
	pub const ALL: [Day; 31] = [
		Day::First,
		Day::Second,
		Day::Third,
		Day::Fourth,
		Day::Fifth,
		Day::Sixth,
		Day::Seventh,
		Day::Eighth,
		Day::Ninth,
		Day::Tenth,
		Day::Eleventh,
		Day::Twelfth,
		Day::Thirteenth,
		Day::Fourteenth,
		Day::Fifteenth,
		Day::Sixteenth,
		Day::Seventeenth,
		Day::Eighteenth,
		Day::Nineteenth,
		Day::Twentieth,
		Day::TwentyFirst,
		Day::TwentySecond,
		Day::TwentyThird,
		Day::TwentyFourth,
		Day::TwentyFifth,
		Day::TwentySixth,
		Day::TwentySeventh,
		Day::TwentyEighth,
		Day::TwentyNinth,
		Day::Thirtieth,
		Day::ThirtyFirst,
	];

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::new(1),  Day::First);
	/// assert_eq!(Day::new(2),  Day::Second);
	/// assert_eq!(Day::new(3),  Day::Third);
	/// assert_eq!(Day::new(4),  Day::Fourth);
	/// assert_eq!(Day::new(5),  Day::Fifth);
	/// assert_eq!(Day::new(6),  Day::Sixth);
	/// assert_eq!(Day::new(7),  Day::Seventh);
	/// assert_eq!(Day::new(8),  Day::Eighth);
	/// assert_eq!(Day::new(9),  Day::Ninth);
	/// assert_eq!(Day::new(10), Day::Tenth);
	/// assert_eq!(Day::new(11), Day::Eleventh);
	/// assert_eq!(Day::new(12), Day::Twelfth);
	/// assert_eq!(Day::new(13), Day::Thirteenth);
	/// assert_eq!(Day::new(14), Day::Fourteenth);
	/// assert_eq!(Day::new(15), Day::Fifteenth);
	/// assert_eq!(Day::new(16), Day::Sixteenth);
	/// assert_eq!(Day::new(17), Day::Seventeenth);
	/// assert_eq!(Day::new(18), Day::Eighteenth);
	/// assert_eq!(Day::new(19), Day::Nineteenth);
	/// assert_eq!(Day::new(20), Day::Twentieth);
	/// assert_eq!(Day::new(21), Day::TwentyFirst);
	/// assert_eq!(Day::new(22), Day::TwentySecond);
	/// assert_eq!(Day::new(23), Day::TwentyThird);
	/// assert_eq!(Day::new(24), Day::TwentyFourth);
	/// assert_eq!(Day::new(25), Day::TwentyFifth);
	/// assert_eq!(Day::new(26), Day::TwentySixth);
	/// assert_eq!(Day::new(27), Day::TwentySeventh);
	/// assert_eq!(Day::new(28), Day::TwentyEighth);
	/// assert_eq!(Day::new(29), Day::TwentyNinth);
	/// assert_eq!(Day::new(30), Day::Thirtieth);
	/// assert_eq!(Day::new(31), Day::ThirtyFirst);
	/// ```
	///
	/// ```rust,should_panic
	/// # use nichi::*;
	/// Day::new(0);
	/// ```
	///
	/// ```rust,should_panic
	/// # use nichi::*;
	/// Day::new(32);
	/// ```
	pub const fn new(day: u8) -> Self {
		match day {
			1  => Self::First,
			2  => Self::Second,
			3  => Self::Third,
			4  => Self::Fourth,
			5  => Self::Fifth,
			6  => Self::Sixth,
			7  => Self::Seventh,
			8  => Self::Eighth,
			9  => Self::Ninth,
			10 => Self::Tenth,
			11 => Self::Eleventh,
			12 => Self::Twelfth,
			13 => Self::Thirteenth,
			14 => Self::Fourteenth,
			15 => Self::Fifteenth,
			16 => Self::Sixteenth,
			17 => Self::Seventeenth,
			18 => Self::Eighteenth,
			19 => Self::Nineteenth,
			20 => Self::Twentieth,
			21 => Self::TwentyFirst,
			22 => Self::TwentySecond,
			23 => Self::TwentyThird,
			24 => Self::TwentyFourth,
			25 => Self::TwentyFifth,
			26 => Self::TwentySixth,
			27 => Self::TwentySeventh,
			28 => Self::TwentyEighth,
			29 => Self::TwentyNinth,
			30 => Self::Thirtieth,
			31 => Self::ThirtyFirst,
			_  => panic!("day is not in-between 1..=12"),
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::new_saturating(1),  Day::First);
	/// assert_eq!(Day::new_saturating(2),  Day::Second);
	/// assert_eq!(Day::new_saturating(3),  Day::Third);
	/// assert_eq!(Day::new_saturating(4),  Day::Fourth);
	/// assert_eq!(Day::new_saturating(5),  Day::Fifth);
	/// assert_eq!(Day::new_saturating(6),  Day::Sixth);
	/// assert_eq!(Day::new_saturating(7),  Day::Seventh);
	/// assert_eq!(Day::new_saturating(8),  Day::Eighth);
	/// assert_eq!(Day::new_saturating(9),  Day::Ninth);
	/// assert_eq!(Day::new_saturating(10), Day::Tenth);
	/// assert_eq!(Day::new_saturating(11), Day::Eleventh);
	/// assert_eq!(Day::new_saturating(12), Day::Twelfth);
	/// assert_eq!(Day::new_saturating(13), Day::Thirteenth);
	/// assert_eq!(Day::new_saturating(14), Day::Fourteenth);
	/// assert_eq!(Day::new_saturating(15), Day::Fifteenth);
	/// assert_eq!(Day::new_saturating(16), Day::Sixteenth);
	/// assert_eq!(Day::new_saturating(17), Day::Seventeenth);
	/// assert_eq!(Day::new_saturating(18), Day::Eighteenth);
	/// assert_eq!(Day::new_saturating(19), Day::Nineteenth);
	/// assert_eq!(Day::new_saturating(20), Day::Twentieth);
	/// assert_eq!(Day::new_saturating(21), Day::TwentyFirst);
	/// assert_eq!(Day::new_saturating(22), Day::TwentySecond);
	/// assert_eq!(Day::new_saturating(23), Day::TwentyThird);
	/// assert_eq!(Day::new_saturating(24), Day::TwentyFourth);
	/// assert_eq!(Day::new_saturating(25), Day::TwentyFifth);
	/// assert_eq!(Day::new_saturating(26), Day::TwentySixth);
	/// assert_eq!(Day::new_saturating(27), Day::TwentySeventh);
	/// assert_eq!(Day::new_saturating(28), Day::TwentyEighth);
	/// assert_eq!(Day::new_saturating(29), Day::TwentyNinth);
	/// assert_eq!(Day::new_saturating(30), Day::Thirtieth);
	/// assert_eq!(Day::new_saturating(31), Day::ThirtyFirst);
	/// ```
	///
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::new_saturating(0),  Day::First);
	/// ```
	///
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::new_saturating(32),  Day::ThirtyFirst);
	/// assert_eq!(Day::new_saturating(33),  Day::ThirtyFirst);
	/// ```
	pub const fn new_saturating(day: u8) -> Self {
		match day {
			0|1 => Self::First,
			2   => Self::Second,
			3   => Self::Third,
			4   => Self::Fourth,
			5   => Self::Fifth,
			6   => Self::Sixth,
			7   => Self::Seventh,
			8   => Self::Eighth,
			9   => Self::Ninth,
			10  => Self::Tenth,
			11  => Self::Eleventh,
			12  => Self::Twelfth,
			13  => Self::Thirteenth,
			14  => Self::Fourteenth,
			15  => Self::Fifteenth,
			16  => Self::Sixteenth,
			17  => Self::Seventeenth,
			18  => Self::Eighteenth,
			19  => Self::Nineteenth,
			20  => Self::Twentieth,
			21  => Self::TwentyFirst,
			22  => Self::TwentySecond,
			23  => Self::TwentyThird,
			24  => Self::TwentyFourth,
			25  => Self::TwentyFifth,
			26  => Self::TwentySixth,
			27  => Self::TwentySeventh,
			28  => Self::TwentyEighth,
			29  => Self::TwentyNinth,
			30  => Self::Thirtieth,
			_   => Self::ThirtyFirst,
		}
	}

	#[inline(always)]
	/// ```rust
	/// # use nichi::*;
	/// // Wraps to 31st
	/// assert_eq!(Day::new_wrapping(0),  Day::ThirtyFirst);
	///
	/// assert_eq!(Day::new_wrapping(1),  Day::First);
	/// assert_eq!(Day::new_wrapping(2),  Day::Second);
	/// assert_eq!(Day::new_wrapping(3),  Day::Third);
	/// assert_eq!(Day::new_wrapping(4),  Day::Fourth);
	/// assert_eq!(Day::new_wrapping(5),  Day::Fifth);
	/// assert_eq!(Day::new_wrapping(6),  Day::Sixth);
	/// assert_eq!(Day::new_wrapping(7),  Day::Seventh);
	/// assert_eq!(Day::new_wrapping(8),  Day::Eighth);
	/// assert_eq!(Day::new_wrapping(9),  Day::Ninth);
	/// assert_eq!(Day::new_wrapping(10), Day::Tenth);
	/// assert_eq!(Day::new_wrapping(11), Day::Eleventh);
	/// assert_eq!(Day::new_wrapping(12), Day::Twelfth);
	/// assert_eq!(Day::new_wrapping(13), Day::Thirteenth);
	/// assert_eq!(Day::new_wrapping(14), Day::Fourteenth);
	/// assert_eq!(Day::new_wrapping(15), Day::Fifteenth);
	/// assert_eq!(Day::new_wrapping(16), Day::Sixteenth);
	/// assert_eq!(Day::new_wrapping(17), Day::Seventeenth);
	/// assert_eq!(Day::new_wrapping(18), Day::Eighteenth);
	/// assert_eq!(Day::new_wrapping(19), Day::Nineteenth);
	/// assert_eq!(Day::new_wrapping(20), Day::Twentieth);
	/// assert_eq!(Day::new_wrapping(21), Day::TwentyFirst);
	/// assert_eq!(Day::new_wrapping(22), Day::TwentySecond);
	/// assert_eq!(Day::new_wrapping(23), Day::TwentyThird);
	/// assert_eq!(Day::new_wrapping(24), Day::TwentyFourth);
	/// assert_eq!(Day::new_wrapping(25), Day::TwentyFifth);
	/// assert_eq!(Day::new_wrapping(26), Day::TwentySixth);
	/// assert_eq!(Day::new_wrapping(27), Day::TwentySeventh);
	/// assert_eq!(Day::new_wrapping(28), Day::TwentyEighth);
	/// assert_eq!(Day::new_wrapping(29), Day::TwentyNinth);
	/// assert_eq!(Day::new_wrapping(30), Day::Thirtieth);
	/// assert_eq!(Day::new_wrapping(31), Day::ThirtyFirst);
	///
	/// // Wraps to 1st, 2nd, etc
	/// assert_eq!(Day::new_wrapping(32), Day::First);
	/// assert_eq!(Day::new_wrapping(33), Day::Second);
	/// assert_eq!(Day::new_wrapping(34), Day::Third);
	/// assert_eq!(Day::new_wrapping(35), Day::Fourth);
	/// assert_eq!(Day::new_wrapping(36), Day::Fifth);
	/// assert_eq!(Day::new_wrapping(37), Day::Sixth);
	/// assert_eq!(Day::new_wrapping(38), Day::Seventh);
	/// assert_eq!(Day::new_wrapping(39), Day::Eighth);
	/// assert_eq!(Day::new_wrapping(40), Day::Ninth);
	/// assert_eq!(Day::new_wrapping(41), Day::Tenth);
	/// assert_eq!(Day::new_wrapping(42), Day::Eleventh);
	/// assert_eq!(Day::new_wrapping(43), Day::Twelfth);
	/// assert_eq!(Day::new_wrapping(44), Day::Thirteenth);
	/// assert_eq!(Day::new_wrapping(45), Day::Fourteenth);
	/// assert_eq!(Day::new_wrapping(46), Day::Fifteenth);
	/// assert_eq!(Day::new_wrapping(47), Day::Sixteenth);
	/// assert_eq!(Day::new_wrapping(48), Day::Seventeenth);
	/// assert_eq!(Day::new_wrapping(49), Day::Eighteenth);
	/// assert_eq!(Day::new_wrapping(50), Day::Nineteenth);
	/// assert_eq!(Day::new_wrapping(51), Day::Twentieth);
	/// assert_eq!(Day::new_wrapping(52), Day::TwentyFirst);
	/// assert_eq!(Day::new_wrapping(53), Day::TwentySecond);
	/// assert_eq!(Day::new_wrapping(54), Day::TwentyThird);
	/// assert_eq!(Day::new_wrapping(55), Day::TwentyFourth);
	/// assert_eq!(Day::new_wrapping(56), Day::TwentyFifth);
	/// assert_eq!(Day::new_wrapping(57), Day::TwentySixth);
	/// assert_eq!(Day::new_wrapping(58), Day::TwentySeventh);
	/// assert_eq!(Day::new_wrapping(59), Day::TwentyEighth);
	/// assert_eq!(Day::new_wrapping(60), Day::TwentyNinth);
	/// assert_eq!(Day::new_wrapping(61), Day::Thirtieth);
	/// assert_eq!(Day::new_wrapping(62), Day::ThirtyFirst);
	/// ```
	pub const fn new_wrapping(day: u8) -> Self {
		match day % 31 {
			1  => Self::First,
			2  => Self::Second,
			3  => Self::Third,
			4  => Self::Fourth,
			5  => Self::Fifth,
			6  => Self::Sixth,
			7  => Self::Seventh,
			8  => Self::Eighth,
			9  => Self::Ninth,
			10 => Self::Tenth,
			11 => Self::Eleventh,
			12 => Self::Twelfth,
			13 => Self::Thirteenth,
			14 => Self::Fourteenth,
			15 => Self::Fifteenth,
			16 => Self::Sixteenth,
			17 => Self::Seventeenth,
			18 => Self::Eighteenth,
			19 => Self::Nineteenth,
			20 => Self::Twentieth,
			21 => Self::TwentyFirst,
			22 => Self::TwentySecond,
			23 => Self::TwentyThird,
			24 => Self::TwentyFourth,
			25 => Self::TwentyFifth,
			26 => Self::TwentySixth,
			27 => Self::TwentySeventh,
			28 => Self::TwentyEighth,
			29 => Self::TwentyNinth,
			30 => Self::Thirtieth,
			_  => Self::ThirtyFirst,
		}
	}

	impl_u8_enum!();

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::First.as_str_num()         , "1");
	/// assert_eq!(Day::Second.as_str_num()        , "2");
	/// assert_eq!(Day::Third.as_str_num()         , "3");
	/// assert_eq!(Day::Fourth.as_str_num()        , "4");
	/// assert_eq!(Day::Fifth.as_str_num()         , "5");
	/// assert_eq!(Day::Sixth.as_str_num()         , "6");
	/// assert_eq!(Day::Seventh.as_str_num()       , "7");
	/// assert_eq!(Day::Eighth.as_str_num()        , "8");
	/// assert_eq!(Day::Ninth.as_str_num()         , "9");
	/// assert_eq!(Day::Tenth.as_str_num()         , "10");
	/// assert_eq!(Day::Eleventh.as_str_num()      , "11");
	/// assert_eq!(Day::Twelfth.as_str_num()       , "12");
	/// assert_eq!(Day::Thirteenth.as_str_num()    , "13");
	/// assert_eq!(Day::Fourteenth.as_str_num()    , "14");
	/// assert_eq!(Day::Fifteenth.as_str_num()     , "15");
	/// assert_eq!(Day::Sixteenth.as_str_num()     , "16");
	/// assert_eq!(Day::Seventeenth.as_str_num()   , "17");
	/// assert_eq!(Day::Eighteenth.as_str_num()    , "18");
	/// assert_eq!(Day::Nineteenth.as_str_num()    , "19");
	/// assert_eq!(Day::Twentieth.as_str_num()     , "20");
	/// assert_eq!(Day::TwentyFirst.as_str_num()   , "21");
	/// assert_eq!(Day::TwentySecond.as_str_num()  , "22");
	/// assert_eq!(Day::TwentyThird.as_str_num()   , "23");
	/// assert_eq!(Day::TwentyFourth.as_str_num()  , "24");
	/// assert_eq!(Day::TwentyFifth.as_str_num()   , "25");
	/// assert_eq!(Day::TwentySixth.as_str_num()   , "26");
	/// assert_eq!(Day::TwentySeventh.as_str_num() , "27");
	/// assert_eq!(Day::TwentyEighth.as_str_num()  , "28");
	/// assert_eq!(Day::TwentyNinth.as_str_num()   , "29");
	/// assert_eq!(Day::Thirtieth.as_str_num()     , "30");
	/// assert_eq!(Day::ThirtyFirst.as_str_num()   , "31");
	/// ```
	pub const fn as_str_num(self) -> &'static str {
		match self {
			Self::First         => 	"1",
			Self::Second        => 	"2",
			Self::Third         => 	"3",
			Self::Fourth        => 	"4",
			Self::Fifth         => 	"5",
			Self::Sixth         => 	"6",
			Self::Seventh       => 	"7",
			Self::Eighth        => 	"8",
			Self::Ninth         => 	"9",
			Self::Tenth         => 	"10",
			Self::Eleventh      => 	"11",
			Self::Twelfth       => 	"12",
			Self::Thirteenth    => 	"13",
			Self::Fourteenth    => 	"14",
			Self::Fifteenth     => 	"15",
			Self::Sixteenth     => 	"16",
			Self::Seventeenth   => 	"17",
			Self::Eighteenth    => 	"18",
			Self::Nineteenth    => 	"19",
			Self::Twentieth     => 	"20",
			Self::TwentyFirst   => 	"21",
			Self::TwentySecond  => 	"22",
			Self::TwentyThird   => 	"23",
			Self::TwentyFourth  => 	"24",
			Self::TwentyFifth   => 	"25",
			Self::TwentySixth   => 	"26",
			Self::TwentySeventh => 	"27",
			Self::TwentyEighth  => 	"28",
			Self::TwentyNinth   => 	"29",
			Self::Thirtieth     => 	"30",
			Self::ThirtyFirst   => 	"31",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::First.as_str_ordinal()          , "First");
	/// assert_eq!(Day::Second.as_str_ordinal()         , "Second");
	/// assert_eq!(Day::Third.as_str_ordinal()          , "Third");
	/// assert_eq!(Day::Fourth.as_str_ordinal()         , "Fourth");
	/// assert_eq!(Day::Fifth.as_str_ordinal()          , "Fifth");
	/// assert_eq!(Day::Sixth.as_str_ordinal()          , "Sixth");
	/// assert_eq!(Day::Seventh.as_str_ordinal()        , "Seventh");
	/// assert_eq!(Day::Eighth.as_str_ordinal()         , "Eighth");
	/// assert_eq!(Day::Ninth.as_str_ordinal()          , "Ninth");
	/// assert_eq!(Day::Tenth.as_str_ordinal()          , "Tenth");
	/// assert_eq!(Day::Eleventh.as_str_ordinal()       , "Eleventh");
	/// assert_eq!(Day::Twelfth.as_str_ordinal()        , "Twelfth");
	/// assert_eq!(Day::Thirteenth.as_str_ordinal()     , "Thirteenth");
	/// assert_eq!(Day::Fourteenth.as_str_ordinal()     , "Fourteenth");
	/// assert_eq!(Day::Fifteenth.as_str_ordinal()      , "Fifteenth");
	/// assert_eq!(Day::Sixteenth.as_str_ordinal()      , "Sixteenth");
	/// assert_eq!(Day::Seventeenth.as_str_ordinal()    , "Seventeenth");
	/// assert_eq!(Day::Eighteenth.as_str_ordinal()     , "Eighteenth");
	/// assert_eq!(Day::Nineteenth.as_str_ordinal()     , "Nineteenth");
	/// assert_eq!(Day::Twentieth.as_str_ordinal()      , "Twentieth");
	/// assert_eq!(Day::TwentyFirst.as_str_ordinal()    , "TwentyFirst");
	/// assert_eq!(Day::TwentySecond.as_str_ordinal()   , "TwentySecond");
	/// assert_eq!(Day::TwentyThird.as_str_ordinal()    , "TwentyThird");
	/// assert_eq!(Day::TwentyFourth.as_str_ordinal()   , "TwentyFourth");
	/// assert_eq!(Day::TwentyFifth.as_str_ordinal()    , "TwentyFifth");
	/// assert_eq!(Day::TwentySixth.as_str_ordinal()    , "TwentySixth");
	/// assert_eq!(Day::TwentySeventh.as_str_ordinal()  , "TwentySeventh");
	/// assert_eq!(Day::TwentyEighth.as_str_ordinal()   , "TwentyEighth");
	/// assert_eq!(Day::TwentyNinth.as_str_ordinal()    , "TwentyNinth");
	/// assert_eq!(Day::Thirtieth.as_str_ordinal()      , "Thirtieth");
	/// assert_eq!(Day::ThirtyFirst.as_str_ordinal()    , "ThirtyFirst");
	/// ```
	pub const fn as_str_ordinal(self) -> &'static str {
		match self {
			Self::First         => 	"First",
			Self::Second        => 	"Second",
			Self::Third         => 	"Third",
			Self::Fourth        => 	"Fourth",
			Self::Fifth         => 	"Fifth",
			Self::Sixth         => 	"Sixth",
			Self::Seventh       => 	"Seventh",
			Self::Eighth        => 	"Eighth",
			Self::Ninth         => 	"Ninth",
			Self::Tenth         => 	"Tenth",
			Self::Eleventh      => 	"Eleventh",
			Self::Twelfth       => 	"Twelfth",
			Self::Thirteenth    => 	"Thirteenth",
			Self::Fourteenth    => 	"Fourteenth",
			Self::Fifteenth     => 	"Fifteenth",
			Self::Sixteenth     => 	"Sixteenth",
			Self::Seventeenth   => 	"Seventeenth",
			Self::Eighteenth    => 	"Eighteenth",
			Self::Nineteenth    => 	"Nineteenth",
			Self::Twentieth     => 	"Twentieth",
			Self::TwentyFirst   => 	"TwentyFirst",
			Self::TwentySecond  => 	"TwentySecond",
			Self::TwentyThird   => 	"TwentyThird",
			Self::TwentyFourth  => 	"TwentyFourth",
			Self::TwentyFifth   => 	"TwentyFifth",
			Self::TwentySixth   => 	"TwentySixth",
			Self::TwentySeventh => 	"TwentySeventh",
			Self::TwentyEighth  => 	"TwentyEighth",
			Self::TwentyNinth   => 	"TwentyNinth",
			Self::Thirtieth     => 	"Thirtieth",
			Self::ThirtyFirst   => 	"ThirtyFirst",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::First.as_str_ordinal_lower()          , "first");
	/// assert_eq!(Day::Second.as_str_ordinal_lower()         , "second");
	/// assert_eq!(Day::Third.as_str_ordinal_lower()          , "third");
	/// assert_eq!(Day::Fourth.as_str_ordinal_lower()         , "fourth");
	/// assert_eq!(Day::Fifth.as_str_ordinal_lower()          , "fifth");
	/// assert_eq!(Day::Sixth.as_str_ordinal_lower()          , "sixth");
	/// assert_eq!(Day::Seventh.as_str_ordinal_lower()        , "seventh");
	/// assert_eq!(Day::Eighth.as_str_ordinal_lower()         , "eighth");
	/// assert_eq!(Day::Ninth.as_str_ordinal_lower()          , "ninth");
	/// assert_eq!(Day::Tenth.as_str_ordinal_lower()          , "tenth");
	/// assert_eq!(Day::Eleventh.as_str_ordinal_lower()       , "eleventh");
	/// assert_eq!(Day::Twelfth.as_str_ordinal_lower()        , "twelfth");
	/// assert_eq!(Day::Thirteenth.as_str_ordinal_lower()     , "thirteenth");
	/// assert_eq!(Day::Fourteenth.as_str_ordinal_lower()     , "fourteenth");
	/// assert_eq!(Day::Fifteenth.as_str_ordinal_lower()      , "fifteenth");
	/// assert_eq!(Day::Sixteenth.as_str_ordinal_lower()      , "sixteenth");
	/// assert_eq!(Day::Seventeenth.as_str_ordinal_lower()    , "seventeenth");
	/// assert_eq!(Day::Eighteenth.as_str_ordinal_lower()     , "eighteenth");
	/// assert_eq!(Day::Nineteenth.as_str_ordinal_lower()     , "nineteenth");
	/// assert_eq!(Day::Twentieth.as_str_ordinal_lower()      , "twentieth");
	/// assert_eq!(Day::TwentyFirst.as_str_ordinal_lower()    , "twentyfirst");
	/// assert_eq!(Day::TwentySecond.as_str_ordinal_lower()   , "twentysecond");
	/// assert_eq!(Day::TwentyThird.as_str_ordinal_lower()    , "twentythird");
	/// assert_eq!(Day::TwentyFourth.as_str_ordinal_lower()   , "twentyfourth");
	/// assert_eq!(Day::TwentyFifth.as_str_ordinal_lower()    , "twentyfifth");
	/// assert_eq!(Day::TwentySixth.as_str_ordinal_lower()    , "twentysixth");
	/// assert_eq!(Day::TwentySeventh.as_str_ordinal_lower()  , "twentyseventh");
	/// assert_eq!(Day::TwentyEighth.as_str_ordinal_lower()   , "twentyeighth");
	/// assert_eq!(Day::TwentyNinth.as_str_ordinal_lower()    , "twentyninth");
	/// assert_eq!(Day::Thirtieth.as_str_ordinal_lower()      , "thirtieth");
	/// assert_eq!(Day::ThirtyFirst.as_str_ordinal_lower()    , "thirtyfirst");
	/// ```
	pub const fn as_str_ordinal_lower(self) -> &'static str {
		match self {
			Self::First         => 	"first",
			Self::Second        => 	"second",
			Self::Third         => 	"third",
			Self::Fourth        => 	"fourth",
			Self::Fifth         => 	"fifth",
			Self::Sixth         => 	"sixth",
			Self::Seventh       => 	"seventh",
			Self::Eighth        => 	"eighth",
			Self::Ninth         => 	"ninth",
			Self::Tenth         => 	"tenth",
			Self::Eleventh      => 	"eleventh",
			Self::Twelfth       => 	"twelfth",
			Self::Thirteenth    => 	"thirteenth",
			Self::Fourteenth    => 	"fourteenth",
			Self::Fifteenth     => 	"fifteenth",
			Self::Sixteenth     => 	"sixteenth",
			Self::Seventeenth   => 	"seventeenth",
			Self::Eighteenth    => 	"eighteenth",
			Self::Nineteenth    => 	"nineteenth",
			Self::Twentieth     => 	"twentieth",
			Self::TwentyFirst   => 	"twentyfirst",
			Self::TwentySecond  => 	"twentysecond",
			Self::TwentyThird   => 	"twentythird",
			Self::TwentyFourth  => 	"twentyfourth",
			Self::TwentyFifth   => 	"twentyfifth",
			Self::TwentySixth   => 	"twentysixth",
			Self::TwentySeventh => 	"twentyseventh",
			Self::TwentyEighth  => 	"twentyeighth",
			Self::TwentyNinth   => 	"twentyninth",
			Self::Thirtieth     => 	"thirtieth",
			Self::ThirtyFirst   => 	"thirtyfirst",
		}
	}

	#[inline]
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::First.as_str_ordinal_upper()          , "FIRST");
	/// assert_eq!(Day::Second.as_str_ordinal_upper()         , "SECOND");
	/// assert_eq!(Day::Third.as_str_ordinal_upper()          , "THIRD");
	/// assert_eq!(Day::Fourth.as_str_ordinal_upper()         , "FOURTH");
	/// assert_eq!(Day::Fifth.as_str_ordinal_upper()          , "FIFTH");
	/// assert_eq!(Day::Sixth.as_str_ordinal_upper()          , "SIXTH");
	/// assert_eq!(Day::Seventh.as_str_ordinal_upper()        , "SEVENTH");
	/// assert_eq!(Day::Eighth.as_str_ordinal_upper()         , "EIGHTH");
	/// assert_eq!(Day::Ninth.as_str_ordinal_upper()          , "NINTH");
	/// assert_eq!(Day::Tenth.as_str_ordinal_upper()          , "TENTH");
	/// assert_eq!(Day::Eleventh.as_str_ordinal_upper()       , "ELEVENTH");
	/// assert_eq!(Day::Twelfth.as_str_ordinal_upper()        , "TWELFTH");
	/// assert_eq!(Day::Thirteenth.as_str_ordinal_upper()     , "THIRTEENTH");
	/// assert_eq!(Day::Fourteenth.as_str_ordinal_upper()     , "FOURTEENTH");
	/// assert_eq!(Day::Fifteenth.as_str_ordinal_upper()      , "FIFTEENTH");
	/// assert_eq!(Day::Sixteenth.as_str_ordinal_upper()      , "SIXTEENTH");
	/// assert_eq!(Day::Seventeenth.as_str_ordinal_upper()    , "SEVENTEENTH");
	/// assert_eq!(Day::Eighteenth.as_str_ordinal_upper()     , "EIGHTEENTH");
	/// assert_eq!(Day::Nineteenth.as_str_ordinal_upper()     , "NINETEENTH");
	/// assert_eq!(Day::Twentieth.as_str_ordinal_upper()      , "TWENTIETH");
	/// assert_eq!(Day::TwentyFirst.as_str_ordinal_upper()    , "TWENTYFIRST");
	/// assert_eq!(Day::TwentySecond.as_str_ordinal_upper()   , "TWENTYSECOND");
	/// assert_eq!(Day::TwentyThird.as_str_ordinal_upper()    , "TWENTYTHIRD");
	/// assert_eq!(Day::TwentyFourth.as_str_ordinal_upper()   , "TWENTYFOURTH");
	/// assert_eq!(Day::TwentyFifth.as_str_ordinal_upper()    , "TWENTYFIFTH");
	/// assert_eq!(Day::TwentySixth.as_str_ordinal_upper()    , "TWENTYSIXTH");
	/// assert_eq!(Day::TwentySeventh.as_str_ordinal_upper()  , "TWENTYSEVENTH");
	/// assert_eq!(Day::TwentyEighth.as_str_ordinal_upper()   , "TWENTYEIGHTH");
	/// assert_eq!(Day::TwentyNinth.as_str_ordinal_upper()    , "TWENTYNINTH");
	/// assert_eq!(Day::Thirtieth.as_str_ordinal_upper()      , "THIRTIETH");
	/// assert_eq!(Day::ThirtyFirst.as_str_ordinal_upper()    , "THIRTYFIRST");
	/// ```
	pub const fn as_str_ordinal_upper(self) -> &'static str {
		match self {
			Day::First         => "FIRST",
			Day::Second        => "SECOND",
			Day::Third         => "THIRD",
			Day::Fourth        => "FOURTH",
			Day::Fifth         => "FIFTH",
			Day::Sixth         => "SIXTH",
			Day::Seventh       => "SEVENTH",
			Day::Eighth        => "EIGHTH",
			Day::Ninth         => "NINTH",
			Day::Tenth         => "TENTH",
			Day::Eleventh      => "ELEVENTH",
			Day::Twelfth       => "TWELFTH",
			Day::Thirteenth    => "THIRTEENTH",
			Day::Fourteenth    => "FOURTEENTH",
			Day::Fifteenth     => "FIFTEENTH",
			Day::Sixteenth     => "SIXTEENTH",
			Day::Seventeenth   => "SEVENTEENTH",
			Day::Eighteenth    => "EIGHTEENTH",
			Day::Nineteenth    => "NINETEENTH",
			Day::Twentieth     => "TWENTIETH",
			Day::TwentyFirst   => "TWENTYFIRST",
			Day::TwentySecond  => "TWENTYSECOND",
			Day::TwentyThird   => "TWENTYTHIRD",
			Day::TwentyFourth  => "TWENTYFOURTH",
			Day::TwentyFifth   => "TWENTYFIFTH",
			Day::TwentySixth   => "TWENTYSIXTH",
			Day::TwentySeventh => "TWENTYSEVENTH",
			Day::TwentyEighth  => "TWENTYEIGHTH",
			Day::TwentyNinth   => "TWENTYNINTH",
			Day::Thirtieth     => "THIRTIETH",
			Day::ThirtyFirst   => "THIRTYFIRST",
		}
	}

	#[inline]
	/// ```
	/// # use nichi::*;
	/// assert_eq!(Day::First.as_str_num_ordinal()         , "1st");
	/// assert_eq!(Day::Second.as_str_num_ordinal()        , "2nd");
	/// assert_eq!(Day::Third.as_str_num_ordinal()         , "3rd");
	/// assert_eq!(Day::Fourth.as_str_num_ordinal()        , "4th");
	/// assert_eq!(Day::Fifth.as_str_num_ordinal()         , "5th");
	/// assert_eq!(Day::Sixth.as_str_num_ordinal()         , "6th");
	/// assert_eq!(Day::Seventh.as_str_num_ordinal()       , "7th");
	/// assert_eq!(Day::Eighth.as_str_num_ordinal()        , "8th");
	/// assert_eq!(Day::Ninth.as_str_num_ordinal()         , "9th");
	/// assert_eq!(Day::Tenth.as_str_num_ordinal()         , "10th");
	/// assert_eq!(Day::Eleventh.as_str_num_ordinal()      , "11th");
	/// assert_eq!(Day::Twelfth.as_str_num_ordinal()       , "12th");
	/// assert_eq!(Day::Thirteenth.as_str_num_ordinal()    , "13th");
	/// assert_eq!(Day::Fourteenth.as_str_num_ordinal()    , "14th");
	/// assert_eq!(Day::Fifteenth.as_str_num_ordinal()     , "15th");
	/// assert_eq!(Day::Sixteenth.as_str_num_ordinal()     , "16th");
	/// assert_eq!(Day::Seventeenth.as_str_num_ordinal()   , "17th");
	/// assert_eq!(Day::Eighteenth.as_str_num_ordinal()    , "18th");
	/// assert_eq!(Day::Nineteenth.as_str_num_ordinal()    , "19th");
	/// assert_eq!(Day::Twentieth.as_str_num_ordinal()     , "20th");
	/// assert_eq!(Day::TwentyFirst.as_str_num_ordinal()   , "21st");
	/// assert_eq!(Day::TwentySecond.as_str_num_ordinal()  , "22nd");
	/// assert_eq!(Day::TwentyThird.as_str_num_ordinal()   , "23rd");
	/// assert_eq!(Day::TwentyFourth.as_str_num_ordinal()  , "24th");
	/// assert_eq!(Day::TwentyFifth.as_str_num_ordinal()   , "25th");
	/// assert_eq!(Day::TwentySixth.as_str_num_ordinal()   , "26th");
	/// assert_eq!(Day::TwentySeventh.as_str_num_ordinal() , "27th");
	/// assert_eq!(Day::TwentyEighth.as_str_num_ordinal()  , "28th");
	/// assert_eq!(Day::TwentyNinth.as_str_num_ordinal()   , "29th");
	/// assert_eq!(Day::Thirtieth.as_str_num_ordinal()     , "30th");
	/// assert_eq!(Day::ThirtyFirst.as_str_num_ordinal()   , "31st");
	/// ```
	pub const fn as_str_num_ordinal(self) -> &'static str {
		match self {
			Self::First         => 	"1st",
			Self::Second        => 	"2nd",
			Self::Third         => 	"3rd",
			Self::Fourth        => 	"4th",
			Self::Fifth         => 	"5th",
			Self::Sixth         => 	"6th",
			Self::Seventh       => 	"7th",
			Self::Eighth        => 	"8th",
			Self::Ninth         => 	"9th",
			Self::Tenth         => 	"10th",
			Self::Eleventh      => 	"11th",
			Self::Twelfth       => 	"12th",
			Self::Thirteenth    => 	"13th",
			Self::Fourteenth    => 	"14th",
			Self::Fifteenth     => 	"15th",
			Self::Sixteenth     => 	"16th",
			Self::Seventeenth   => 	"17th",
			Self::Eighteenth    => 	"18th",
			Self::Nineteenth    => 	"19th",
			Self::Twentieth     => 	"20th",
			Self::TwentyFirst   => 	"21st",
			Self::TwentySecond  => 	"22nd",
			Self::TwentyThird   => 	"23rd",
			Self::TwentyFourth  => 	"24th",
			Self::TwentyFifth   => 	"25th",
			Self::TwentySixth   => 	"26th",
			Self::TwentySeventh => 	"27th",
			Self::TwentyEighth  => 	"28th",
			Self::TwentyNinth   => 	"29th",
			Self::Thirtieth     => 	"30th",
			Self::ThirtyFirst   => 	"31st",
		}
	}

	#[inline]
	/// ```
	/// # use nichi::*;
	/// assert_eq!(Day::First.as_str_num_ordinal_upper()         , "1ST");
	/// assert_eq!(Day::Second.as_str_num_ordinal_upper()        , "2ND");
	/// assert_eq!(Day::Third.as_str_num_ordinal_upper()         , "3RD");
	/// assert_eq!(Day::Fourth.as_str_num_ordinal_upper()        , "4TH");
	/// assert_eq!(Day::Fifth.as_str_num_ordinal_upper()         , "5TH");
	/// assert_eq!(Day::Sixth.as_str_num_ordinal_upper()         , "6TH");
	/// assert_eq!(Day::Seventh.as_str_num_ordinal_upper()       , "7TH");
	/// assert_eq!(Day::Eighth.as_str_num_ordinal_upper()        , "8TH");
	/// assert_eq!(Day::Ninth.as_str_num_ordinal_upper()         , "9TH");
	/// assert_eq!(Day::Tenth.as_str_num_ordinal_upper()         , "10TH");
	/// assert_eq!(Day::Eleventh.as_str_num_ordinal_upper()      , "11TH");
	/// assert_eq!(Day::Twelfth.as_str_num_ordinal_upper()       , "12TH");
	/// assert_eq!(Day::Thirteenth.as_str_num_ordinal_upper()    , "13TH");
	/// assert_eq!(Day::Fourteenth.as_str_num_ordinal_upper()    , "14TH");
	/// assert_eq!(Day::Fifteenth.as_str_num_ordinal_upper()     , "15TH");
	/// assert_eq!(Day::Sixteenth.as_str_num_ordinal_upper()     , "16TH");
	/// assert_eq!(Day::Seventeenth.as_str_num_ordinal_upper()   , "17TH");
	/// assert_eq!(Day::Eighteenth.as_str_num_ordinal_upper()    , "18TH");
	/// assert_eq!(Day::Nineteenth.as_str_num_ordinal_upper()    , "19TH");
	/// assert_eq!(Day::Twentieth.as_str_num_ordinal_upper()     , "20TH");
	/// assert_eq!(Day::TwentyFirst.as_str_num_ordinal_upper()   , "21ST");
	/// assert_eq!(Day::TwentySecond.as_str_num_ordinal_upper()  , "22ND");
	/// assert_eq!(Day::TwentyThird.as_str_num_ordinal_upper()   , "23RD");
	/// assert_eq!(Day::TwentyFourth.as_str_num_ordinal_upper()  , "24TH");
	/// assert_eq!(Day::TwentyFifth.as_str_num_ordinal_upper()   , "25TH");
	/// assert_eq!(Day::TwentySixth.as_str_num_ordinal_upper()   , "26TH");
	/// assert_eq!(Day::TwentySeventh.as_str_num_ordinal_upper() , "27TH");
	/// assert_eq!(Day::TwentyEighth.as_str_num_ordinal_upper()  , "28TH");
	/// assert_eq!(Day::TwentyNinth.as_str_num_ordinal_upper()   , "29TH");
	/// assert_eq!(Day::Thirtieth.as_str_num_ordinal_upper()     , "30TH");
	/// assert_eq!(Day::ThirtyFirst.as_str_num_ordinal_upper()   , "31ST");
	/// ```
	pub const fn as_str_num_ordinal_upper(self) -> &'static str {
		match self {
			Self::First         => 	"1ST",
			Self::Second        => 	"2ND",
			Self::Third         => 	"3RD",
			Self::Fourth        => 	"4TH",
			Self::Fifth         => 	"5TH",
			Self::Sixth         => 	"6TH",
			Self::Seventh       => 	"7TH",
			Self::Eighth        => 	"8TH",
			Self::Ninth         => 	"9TH",
			Self::Tenth         => 	"10TH",
			Self::Eleventh      => 	"11TH",
			Self::Twelfth       => 	"12TH",
			Self::Thirteenth    => 	"13TH",
			Self::Fourteenth    => 	"14TH",
			Self::Fifteenth     => 	"15TH",
			Self::Sixteenth     => 	"16TH",
			Self::Seventeenth   => 	"17TH",
			Self::Eighteenth    => 	"18TH",
			Self::Nineteenth    => 	"19TH",
			Self::Twentieth     => 	"20TH",
			Self::TwentyFirst   => 	"21ST",
			Self::TwentySecond  => 	"22ND",
			Self::TwentyThird   => 	"23RD",
			Self::TwentyFourth  => 	"24TH",
			Self::TwentyFifth   => 	"25TH",
			Self::TwentySixth   => 	"26TH",
			Self::TwentySeventh => 	"27TH",
			Self::TwentyEighth  => 	"28TH",
			Self::TwentyNinth   => 	"29TH",
			Self::Thirtieth     => 	"30TH",
			Self::ThirtyFirst   => 	"31ST",
		}
	}

	/// Create a [`Day`] by parsing a [`&str`]
	///
	/// Valid input strings are anything returned by
	/// - [`Day::as_str_num`]
	/// - [`Day::as_str_ordinal`]
	/// - [`Day::as_str_num_ordinal`]
	///
	/// These cases are covered:
	/// - `lowercase`
	/// - `UPPERCASE`
	/// - `CamelCase`
	///
	/// For example:
	///
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::from_str("21").unwrap(),          Day::TwentyFirst);
	/// assert_eq!(Day::from_str("21st").unwrap(),        Day::TwentyFirst);
	/// assert_eq!(Day::from_str("21ST").unwrap(),        Day::TwentyFirst);
	/// assert_eq!(Day::from_str("twentyfirst").unwrap(), Day::TwentyFirst);
	/// assert_eq!(Day::from_str("TwentyFirst").unwrap(), Day::TwentyFirst);
	/// assert_eq!(Day::from_str("TWENTYFIRST").unwrap(), Day::TwentyFirst);
	/// ```
	///
	/// ## Examples
	/// ```rust
	/// # use nichi::*;
	/// assert_eq!(Day::from_str("1").unwrap(),     Day::First);
	/// assert_eq!(Day::from_str("1st").unwrap(),   Day::First);
	/// assert_eq!(Day::from_str("1ST").unwrap(),   Day::First);
	/// assert_eq!(Day::from_str("first").unwrap(), Day::First);
	/// assert_eq!(Day::from_str("First").unwrap(), Day::First);
	/// assert_eq!(Day::from_str("FIRST").unwrap(), Day::First);
	///
	/// assert_eq!(Day::from_str("2").unwrap(),      Day::Second);
	/// assert_eq!(Day::from_str("2nd").unwrap(),    Day::Second);
	/// assert_eq!(Day::from_str("2ND").unwrap(),    Day::Second);
	/// assert_eq!(Day::from_str("second").unwrap(), Day::Second);
	/// assert_eq!(Day::from_str("Second").unwrap(), Day::Second);
	/// assert_eq!(Day::from_str("SECOND").unwrap(), Day::Second);
	///
	/// assert_eq!(Day::from_str("3").unwrap(),     Day::Third);
	/// assert_eq!(Day::from_str("3rd").unwrap(),   Day::Third);
	/// assert_eq!(Day::from_str("3RD").unwrap(),   Day::Third);
	/// assert_eq!(Day::from_str("third").unwrap(), Day::Third);
	/// assert_eq!(Day::from_str("Third").unwrap(), Day::Third);
	/// assert_eq!(Day::from_str("THIRD").unwrap(), Day::Third);
	///
	/// assert_eq!(Day::from_str("10").unwrap(),    Day::Tenth);
	/// assert_eq!(Day::from_str("10th").unwrap(),  Day::Tenth);
	/// assert_eq!(Day::from_str("10TH").unwrap(),  Day::Tenth);
	/// assert_eq!(Day::from_str("tenth").unwrap(), Day::Tenth);
	/// assert_eq!(Day::from_str("Tenth").unwrap(), Day::Tenth);
	/// assert_eq!(Day::from_str("TENTH").unwrap(), Day::Tenth);
	///
	/// assert_eq!(Day::from_str("31").unwrap(),          Day::ThirtyFirst);
	/// assert_eq!(Day::from_str("31st").unwrap(),        Day::ThirtyFirst);
	/// assert_eq!(Day::from_str("31ST").unwrap(),        Day::ThirtyFirst);
	/// assert_eq!(Day::from_str("thirtyfirst").unwrap(), Day::ThirtyFirst);
	/// assert_eq!(Day::from_str("ThirtyFirst").unwrap(), Day::ThirtyFirst);
	/// assert_eq!(Day::from_str("THIRTYFIRST").unwrap(), Day::ThirtyFirst);
	/// ```
	pub const fn from_str(s: &str) -> Option<Self> {
		let bytes = s.as_bytes();
		let len   = bytes.len();

		if len == 0 {
			return None;
		}

		if len <= 4 {
			match bytes {
				b"1"  |  b"1st" | b"1ST"  => Some(Day::First),
				b"2"  |  b"2nd" | b"2ND"  => Some(Day::Second),
				b"3"  |  b"3rd" | b"3RD"  => Some(Day::Third),
				b"4"  |  b"4th" | b"4TH"  => Some(Day::Fourth),
				b"5"  |  b"5th" | b"5TH"  => Some(Day::Fifth),
				b"6"  |  b"6th" | b"6TH"  => Some(Day::Sixth),
				b"7"  |  b"7th" | b"7TH"  => Some(Day::Seventh),
				b"8"  |  b"8th" | b"8TH"  => Some(Day::Eighth),
				b"9"  |  b"9th" | b"9TH"  => Some(Day::Ninth),
				b"10" | b"10th" | b"10TH" => Some(Day::Tenth),
				b"11" | b"11th" | b"11TH" => Some(Day::Eleventh),
				b"12" | b"12th" | b"12TH" => Some(Day::Twelfth),
				b"13" | b"13th" | b"13TH" => Some(Day::Thirteenth),
				b"14" | b"14th" | b"14TH" => Some(Day::Fourteenth),
				b"15" | b"15th" | b"15TH" => Some(Day::Fifteenth),
				b"16" | b"16th" | b"16TH" => Some(Day::Sixteenth),
				b"17" | b"17th" | b"17TH" => Some(Day::Seventeenth),
				b"18" | b"18th" | b"18TH" => Some(Day::Eighteenth),
				b"19" | b"19th" | b"19TH" => Some(Day::Nineteenth),
				b"20" | b"20th" | b"20TH" => Some(Day::Twentieth),
				b"21" | b"21st" | b"21ST" => Some(Day::TwentyFirst),
				b"22" | b"22nd" | b"22ND" => Some(Day::TwentySecond),
				b"23" | b"23rd" | b"23RD" => Some(Day::TwentyThird),
				b"24" | b"24th" | b"24TH" => Some(Day::TwentyFourth),
				b"25" | b"25th" | b"25TH" => Some(Day::TwentyFifth),
				b"26" | b"26th" | b"26TH" => Some(Day::TwentySixth),
				b"27" | b"27th" | b"27TH" => Some(Day::TwentySeventh),
				b"28" | b"28th" | b"28TH" => Some(Day::TwentyEighth),
				b"29" | b"29th" | b"29TH" => Some(Day::TwentyNinth),
				b"30" | b"30th" | b"30TH" => Some(Day::Thirtieth),
				b"31" | b"31st" | b"31ST" => Some(Day::ThirtyFirst),
				_ => None,
			}
		} else {
			match bytes {
				b"First"         | b"first"         | b"FIRST"         => Some(Self::First),
				b"Second"        | b"second"        | b"SECOND"        => Some(Self::Second),
				b"Third"         | b"third"         | b"THIRD"         => Some(Self::Third),
				b"Fourth"        | b"fourth"        | b"FOURTH"        => Some(Self::Fourth),
				b"Fifth"         | b"fifth"         | b"FIFTH"         => Some(Self::Fifth),
				b"Sixth"         | b"sixth"         | b"SIXTH"         => Some(Self::Sixth),
				b"Seventh"       | b"seventh"       | b"SEVENTH"       => Some(Self::Seventh),
				b"Eighth"        | b"eighth"        | b"EIGHTH"        => Some(Self::Eighth),
				b"Ninth"         | b"ninth"         | b"NINTH"         => Some(Self::Ninth),
				b"Tenth"         | b"tenth"         | b"TENTH"         => Some(Self::Tenth),
				b"Eleventh"      | b"eleventh"      | b"ELEVENTH"      => Some(Self::Eleventh),
				b"Twelfth"       | b"twelfth"       | b"TWELFTH"       => Some(Self::Twelfth),
				b"Thirteenth"    | b"thirteenth"    | b"THIRTEENTH"    => Some(Self::Thirteenth),
				b"Fourteenth"    | b"fourteenth"    | b"FOURTEENTH"    => Some(Self::Fourteenth),
				b"Fifteenth"     | b"fifteenth"     | b"FIFTEENTH"     => Some(Self::Fifteenth),
				b"Sixteenth"     | b"sixteenth"     | b"SIXTEENTH"     => Some(Self::Sixteenth),
				b"Seventeenth"   | b"seventeenth"   | b"SEVENTEENTH"   => Some(Self::Seventeenth),
				b"Eighteenth"    | b"eighteenth"    | b"EIGHTEENTH"    => Some(Self::Eighteenth),
				b"Nineteenth"    | b"nineteenth"    | b"NINETEENTH"    => Some(Self::Nineteenth),
				b"Twentieth"     | b"twentieth"     | b"TWENTIETH"     => Some(Self::Twentieth),
				b"TwentyFirst"   | b"twentyfirst"   | b"TWENTYFIRST"   => Some(Self::TwentyFirst),
				b"TwentySecond"  | b"twentysecond"  | b"TWENTYSECOND"  => Some(Self::TwentySecond),
				b"TwentyThird"   | b"twentythird"   | b"TWENTYTHIRD"   => Some(Self::TwentyThird),
				b"TwentyFourth"  | b"twentyfourth"  | b"TWENTYFOURTH"  => Some(Self::TwentyFourth),
				b"TwentyFifth"   | b"Twentyfifth"   | b"TWENTYFIFTH"   => Some(Self::TwentyFifth),
				b"TwentySixth"   | b"Twentysixth"   | b"TWENTYSIXTH"   => Some(Self::TwentySixth),
				b"TwentySeventh" | b"Twentyseventh" | b"TWENTYSEVENTH" => Some(Self::TwentySeventh),
				b"TwentyEighth"  | b"Twentyeighth"  | b"TWENTYEIGHTH"  => Some(Self::TwentyEighth),
				b"TwentyNinth"   | b"Twentyninth"   | b"TWENTYNINTH"   => Some(Self::TwentyNinth),
				b"Thirtieth"     | b"thirtieth"     | b"THIRTIETH"     => Some(Self::Thirtieth),
				b"ThirtyFirst"   | b"thirtyfirst"   | b"THIRTYFIRST"   => Some(Self::ThirtyFirst),
				_ => None,
			}
		}
	}
}

//---------------------------------------------------------------------------------------------------- Impl
