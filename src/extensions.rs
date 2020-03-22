use chrono::{NaiveDateTime, Duration, Timelike, Datelike};
use std::ops::{Add, Sub};
use crate::models::SimpleDate;

pub trait Utils {
    fn minus_one_hour(&self) -> Self;
    fn plus_one_hour(&self) -> Self;
    fn minus_one_day(&self) -> Self;
    fn plus_one_day(&self) -> Self;
    fn plus_one_month(&self) -> Self;
    fn minus_one_month(&self) -> Self;
}

impl Utils for NaiveDateTime {
    fn minus_one_hour(&self) -> NaiveDateTime {
        return self.sub(Duration::hours(1));
    }

    fn plus_one_hour(&self) -> NaiveDateTime {
        return self.add(Duration::hours(1));
    }

    fn minus_one_day(&self) -> Self {
        return self.sub(Duration::days(1));
    }

    fn plus_one_day(&self) -> Self {
        return self.add(Duration::days(1));
    }

    fn plus_one_month(&self) -> Self {
        let days = days_in_month(self.month() as u8, self.year() as u32);
        let start = self.with_day(1).expect("Day 1 invalid (plus one month)");
        return start.add(Duration::days((days + 1) as i64));
    }

    fn minus_one_month(&self) -> Self {
        return self.with_day(1).expect("Day 1 invalid (minus one month [a])")
            .sub(Duration::days(1))
            .with_day(1).expect("Day 1 invalid (minus one month [b])");
    }
}

pub fn is_leap_year(year: u32) -> bool {
    return (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
}

/// Returns days in a month (accounting for leap years)
/// Month are 1 based (i.e Jan = 1, Dec = 12)
pub fn days_in_month(month: u8, year: u32) -> u8 {
    match month {
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => panic!("Invalid month: {}", month)
    }
}

impl From<SimpleDate> for NaiveDateTime {
    fn from(value: SimpleDate) -> Self {
        NaiveDateTime::from_timestamp(0, 0)
            .with_year(value.year as i32).expect("Bad year")
            .with_ordinal(value.day as u32).expect("Bad day")
            .with_hour(value.hour as u32).expect("Bad hour")
    }
}

impl From<NaiveDateTime> for SimpleDate {
    fn from(date: NaiveDateTime) -> Self {
        return SimpleDate::new(date.year() as u16, date.ordinal() as u16, date.hour() as u8);
    }
}

fn unit() {
    return ();
}

pub trait MapToUnit<E> {
    fn map_to_unit(self) -> Result<(), E>;
}

impl<U, E> MapToUnit<E> for Result<U, E> {
    fn map_to_unit(self) -> Result<(), E> {
        return self.map(|_| unit());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(1, 1900), 31, "Jan has 31");
        assert_eq!(days_in_month(2, 1900), 31, "Feb has 28 on non leap years");
        assert_eq!(days_in_month(3, 1900), 31, "Mar has 31");
        assert_eq!(days_in_month(4, 1900), 31, "Apr has 30");
        assert_eq!(days_in_month(5, 1900), 31, "May has 31");
        assert_eq!(days_in_month(6, 1900), 31, "Jun has 30");
        assert_eq!(days_in_month(7, 1900), 31, "Jul has 31");
        assert_eq!(days_in_month(8, 1900), 31, "Aug has 31");
        assert_eq!(days_in_month(9, 1900), 31, "Sep has 30");
        assert_eq!(days_in_month(10, 1900), 31, "Oct has 31");
        assert_eq!(days_in_month(11, 1900), 31, "Nov has 30");
        assert_eq!(days_in_month(12, 1900), 31, "Dec has 31");
        assert_eq!(days_in_month(1, 2000), 31, "Jan always has 31");
        assert_eq!(days_in_month(2, 2000), 31, "Feb has 29 on leap years");
    }

    #[test]
    fn test_leap_year() {
        assert!(!is_leap_year(1900), "is not leap year");
        assert!(!is_leap_year(2100), "is not leap year");
        assert!(!is_leap_year(2101), "is not leap year");
        assert!(is_leap_year(2000), "is leap year");
        assert!(is_leap_year(2016), "is leap year");
    }
}