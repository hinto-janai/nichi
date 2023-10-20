# `nichi`
[![CI](https://github.com/hinto-janai/nichi/actions/workflows/ci.yml/badge.svg)](https://github.com/hinto-janai/nichi/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/nichi.svg)](https://crates.io/crates/nichi) [![docs.rs](https://docs.rs/nichi/badge.svg)](https://docs.rs/nichi)

Date library.

## PartialEq
```rust
use nichi::*;

// PartialEq.
let date1 = Date::new(2000, 12, 25);
let date2 = Date::new_typed(
	Year(2000),
	Month::December,
	Day::TwentyFifth,
);

assert_eq!(date1, date2);
```

## Weekday calculation
```rust
use nichi::*;

// Christmas in 2000 was on a Monday.
assert_eq!(Date::new(2000, 12, 25).weekday(), Weekday::Monday);
```

## Unix calculation
```rust
# use nichi::*;
let date = Date::new(2023, 10, 20);
assert_eq!(date.as_unix(), 1697760000);
assert_eq!(date, Date::from_unix(date.as_unix()));
```

## String parsing
```rust
use nichi::*;

assert_eq!(Day::from_str("1st").unwrap(),   Day::First);
assert_eq!(Day::from_str("first").unwrap(), Day::First);
assert_eq!(Day::from_str("FIRST").unwrap(), Day::First);
assert_eq!(Day::from_str("10TH").unwrap(),  Day::Tenth);
assert_eq!(Day::from_str("tenth").unwrap(), Day::Tenth);
assert_eq!(Day::from_str("Tenth").unwrap(), Day::Tenth);

assert_eq!(Weekday::from_str("Tuesday").unwrap(),   Weekday::Tuesday);
assert_eq!(Weekday::from_str("wed").unwrap(),       Weekday::Wednesday);
assert_eq!(Weekday::from_str("WEDNESDAY").unwrap(), Weekday::Wednesday);
assert_eq!(Weekday::from_str("Wednesday").unwrap(), Weekday::Wednesday);
assert_eq!(Weekday::from_str("THURS").unwrap(),     Weekday::Thursday);
assert_eq!(Weekday::from_str("thurs").unwrap(),     Weekday::Thursday);
assert_eq!(Weekday::from_str("THUR").unwrap(),      Weekday::Thursday);
assert_eq!(Weekday::from_str("thursday").unwrap(),  Weekday::Thursday);

assert_eq!(Month::from_str("January").unwrap(),   Month::January);
assert_eq!(Month::from_str("February").unwrap(),  Month::February);
assert_eq!(Month::from_str("FEBRUARY").unwrap(),  Month::February);
assert_eq!(Month::from_str("JUN").unwrap(),       Month::June);
assert_eq!(Month::from_str("July").unwrap(),      Month::July);
assert_eq!(Month::from_str("Jul").unwrap(),       Month::July);
assert_eq!(Month::from_str("NOVEMBER").unwrap(),  Month::November);
assert_eq!(Month::from_str("nov").unwrap(),       Month::November);
assert_eq!(Month::from_str("DEC").unwrap(),       Month::December);
```