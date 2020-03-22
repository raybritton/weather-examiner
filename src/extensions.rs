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

    //Sets day to 1
    fn plus_one_month(&self) -> Self {
        let days = days_in_month(self.month() as u8, self.year() as u32);
        let start = self.with_day(1).expect("Day 1 invalid (plus one month)");
        return start.add(Duration::days(days as i64));
    }

    //Sets day to 1
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
    use std::str::FromStr;

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(1, 1900), 31, "Jan has 31");
        assert_eq!(days_in_month(2, 1900), 28, "Feb has 28 on non leap years");
        assert_eq!(days_in_month(3, 1900), 31, "Mar has 31");
        assert_eq!(days_in_month(4, 1900), 30, "Apr has 30");
        assert_eq!(days_in_month(5, 1900), 31, "May has 31");
        assert_eq!(days_in_month(6, 1900), 30, "Jun has 30");
        assert_eq!(days_in_month(7, 1900), 31, "Jul has 31");
        assert_eq!(days_in_month(8, 1900), 31, "Aug has 31");
        assert_eq!(days_in_month(9, 1900), 30, "Sep has 30");
        assert_eq!(days_in_month(10, 1900), 31, "Oct has 31");
        assert_eq!(days_in_month(11, 1900), 30, "Nov has 30");
        assert_eq!(days_in_month(12, 1900), 31, "Dec has 31");
        assert_eq!(days_in_month(1, 2000), 31, "Jan always has 31");
        assert_eq!(days_in_month(2, 2000), 29, "Feb has 29 on leap years");
    }

    #[test]
    fn test_leap_year() {
        assert!(!is_leap_year(1900), "is not leap year");
        assert!(!is_leap_year(2100), "is not leap year");
        assert!(!is_leap_year(2101), "is not leap year");
        assert!(is_leap_year(2000), "is leap year");
        assert!(is_leap_year(2016), "is leap year");
    }





    #[test]
    fn test_one_day_middle_month() {
        let middle_of_month = NaiveDateTime::from_str("2015-09-18T23:56:04").unwrap();
        let middle_plus_one = middle_of_month.plus_one_day();
        let middle_minus_one = middle_of_month.minus_one_day();
        assert_eq!(middle_plus_one.hour(), 23, "+ h");
        assert_eq!(middle_plus_one.day(), 19, "+ d");
        assert_eq!(middle_plus_one.month(), 9, "+ m");
        assert_eq!(middle_minus_one.hour(), 23, "- h");
        assert_eq!(middle_minus_one.day(), 17, "- d");
        assert_eq!(middle_minus_one.month(), 9, "- m");
    }

    #[test]
    fn test_one_day_beginning_month() {
        let april_start = NaiveDateTime::from_str("2015-04-1T23:56:04").unwrap();
        let april_start_plus_one = april_start.plus_one_day();
        let april_start_minus_one = april_start.minus_one_day();
        assert_eq!(april_start_plus_one.hour(), 23, "april + h");
        assert_eq!(april_start_plus_one.day(), 2, "april + d");
        assert_eq!(april_start_plus_one.month(), 4, "april + m");
        assert_eq!(april_start_minus_one.hour(), 23, "april - h");
        assert_eq!(april_start_minus_one.day(), 31, "april - d");
        assert_eq!(april_start_minus_one.month(), 3, "april - m");

        //non leap
        let march_start = NaiveDateTime::from_str("2015-03-1T23:56:04").unwrap();
        let march_start_plus_one = march_start.plus_one_day();
        let march_start_minus_one = march_start.minus_one_day();
        assert_eq!(march_start_plus_one.hour(), 23, "march + h");
        assert_eq!(march_start_plus_one.day(), 2, "march + d");
        assert_eq!(march_start_plus_one.month(), 3, "march + m");
        assert_eq!(march_start_minus_one.hour(), 23, "march - h");
        assert_eq!(march_start_minus_one.day(), 28, "march - d");
        assert_eq!(march_start_minus_one.month(), 2, "march - m");

        //leap
        let march_leap_start = NaiveDateTime::from_str("2020-03-1T23:56:04").unwrap();
        let march_leap_start_plus_one = march_leap_start.plus_one_day();
        let march_leap_start_minus_one = march_leap_start.minus_one_day();
        assert_eq!(march_leap_start_plus_one.hour(), 23, "leap march + h");
        assert_eq!(march_leap_start_plus_one.day(), 2, "leap march + d");
        assert_eq!(march_leap_start_plus_one.month(), 3, "leap march + m");
        assert_eq!(march_leap_start_minus_one.hour(), 23, "leap march - h");
        assert_eq!(march_leap_start_minus_one.day(), 29, "leap march - d");
        assert_eq!(march_leap_start_minus_one.month(), 2, "leap march - m");
    }

    #[test]
    fn test_one_day_end_month() {
        let april_end = NaiveDateTime::from_str("2015-04-30T23:56:04").unwrap();
        let april_end_plus_one = april_end.plus_one_day();
        let april_end_minus_one = april_end.minus_one_day();
        assert_eq!(april_end_plus_one.hour(), 23, "april + h");
        assert_eq!(april_end_plus_one.day(), 1, "april + d");
        assert_eq!(april_end_plus_one.month(), 5, "april + m");
        assert_eq!(april_end_minus_one.hour(), 23, "april - h");
        assert_eq!(april_end_minus_one.day(), 29, "april - d");
        assert_eq!(april_end_minus_one.month(), 4, "april - m");

        //non leap
        let feb_end = NaiveDateTime::from_str("2015-02-28T23:56:04").unwrap();
        let feb_end_plus_one = feb_end.plus_one_day();
        let feb_end_minus_one = feb_end.minus_one_day();
        assert_eq!(feb_end_plus_one.hour(), 23, "feb + h");
        assert_eq!(feb_end_plus_one.day(), 1, "feb + d");
        assert_eq!(feb_end_plus_one.month(), 3, "feb + m");
        assert_eq!(feb_end_minus_one.hour(), 23, "feb - h");
        assert_eq!(feb_end_minus_one.day(), 27, "feb - d");
        assert_eq!(feb_end_minus_one.month(), 2, "feb - m");

        //leap
        let feb_leap_end = NaiveDateTime::from_str("2020-02-29T23:56:04").unwrap();
        let feb_leap_end_plus_one = feb_leap_end.plus_one_day();
        let feb_leap_end_minus_one = feb_leap_end.minus_one_day();
        assert_eq!(feb_leap_end_plus_one.hour(), 23, "leap feb + h");
        assert_eq!(feb_leap_end_plus_one.day(), 1, "leap feb + d");
        assert_eq!(feb_leap_end_plus_one.month(), 3, "leap feb + m");
        assert_eq!(feb_leap_end_minus_one.hour(), 23, "leap feb - h");
        assert_eq!(feb_leap_end_minus_one.day(), 28, "leap feb - d");
        assert_eq!(feb_leap_end_minus_one.month(), 2, "leap feb - m");
    }





    #[test]
    fn test_one_hour_middle_day() {
        let middle_of_month = NaiveDateTime::from_str("2015-09-18T13:56:04").unwrap();
        let middle_plus_one = middle_of_month.plus_one_hour();
        let middle_minus_one = middle_of_month.minus_one_hour();
        assert_eq!(middle_plus_one.minute(), 56, "+ min");
        assert_eq!(middle_plus_one.hour(), 14, "+ h");
        assert_eq!(middle_plus_one.day(), 18, "+ d");
        assert_eq!(middle_plus_one.month(), 9, "+ m");
        assert_eq!(middle_minus_one.minute(), 56, "- min");
        assert_eq!(middle_minus_one.hour(), 12, "- h");
        assert_eq!(middle_minus_one.day(), 18, "- d");
        assert_eq!(middle_minus_one.month(), 9, "- m");
    }

    #[test]
    fn test_one_hour_beginning_day() {
        let april_start = NaiveDateTime::from_str("2015-04-1T0:56:04").unwrap();
        let april_start_plus_one = april_start.plus_one_hour();
        let april_start_minus_one = april_start.minus_one_hour();
        assert_eq!(april_start_plus_one.minute(), 56, "april + min");
        assert_eq!(april_start_plus_one.hour(), 1, "april + h");
        assert_eq!(april_start_plus_one.day(), 1, "april + d");
        assert_eq!(april_start_plus_one.month(), 4, "april + m");
        assert_eq!(april_start_minus_one.minute(), 56, "april - min");
        assert_eq!(april_start_minus_one.hour(), 23, "april - h");
        assert_eq!(april_start_minus_one.day(), 31, "april - d");
        assert_eq!(april_start_minus_one.month(), 3, "april - m");

        //non leap
        let march_start = NaiveDateTime::from_str("2015-03-1T0:56:04").unwrap();
        let march_start_plus_one = march_start.plus_one_hour();
        let march_start_minus_one = march_start.minus_one_hour();
        assert_eq!(march_start_plus_one.minute(), 56, "march + min");
        assert_eq!(march_start_plus_one.hour(), 1, "march + h");
        assert_eq!(march_start_plus_one.day(), 1, "march + d");
        assert_eq!(march_start_plus_one.month(), 3, "march + m");
        assert_eq!(march_start_minus_one.minute(), 56, "march - min");
        assert_eq!(march_start_minus_one.hour(), 23, "march - h");
        assert_eq!(march_start_minus_one.day(), 28, "march - d");
        assert_eq!(march_start_minus_one.month(), 2, "march - m");

        //leap
        let march_leap_start = NaiveDateTime::from_str("2020-03-1T0:56:04").unwrap();
        let march_leap_start_plus_one = march_leap_start.plus_one_hour();
        let march_leap_start_minus_one = march_leap_start.minus_one_hour();
        assert_eq!(march_leap_start_plus_one.minute(), 56, "leap march + min");
        assert_eq!(march_leap_start_plus_one.hour(), 1, "leap march + h");
        assert_eq!(march_leap_start_plus_one.day(), 1, "leap march + d");
        assert_eq!(march_leap_start_plus_one.month(), 3, "leap march + m");
        assert_eq!(march_leap_start_minus_one.minute(), 56, "leap march - min");
        assert_eq!(march_leap_start_minus_one.hour(), 23, "leap march - h");
        assert_eq!(march_leap_start_minus_one.day(), 29, "leap march - d");
        assert_eq!(march_leap_start_minus_one.month(), 2, "leap march - m");
    }

    #[test]
    fn test_one_hour_end_day() {
        let april_end = NaiveDateTime::from_str("2015-04-30T23:56:04").unwrap();
        let april_end_plus_one = april_end.plus_one_hour();
        let april_end_minus_one = april_end.minus_one_hour();
        assert_eq!(april_end_plus_one.minute(), 56, "april + min");
        assert_eq!(april_end_plus_one.hour(), 0, "april + h");
        assert_eq!(april_end_plus_one.day(), 1, "april + d");
        assert_eq!(april_end_plus_one.month(), 5, "april + m");
        assert_eq!(april_end_minus_one.minute(), 56, "april - min");
        assert_eq!(april_end_minus_one.hour(), 22, "april - h");
        assert_eq!(april_end_minus_one.day(), 30, "april - d");
        assert_eq!(april_end_minus_one.month(), 4, "april - m");

        //non leap
        let feb_end = NaiveDateTime::from_str("2015-02-28T23:56:04").unwrap();
        let feb_end_plus_one = feb_end.plus_one_hour();
        let feb_end_minus_one = feb_end.minus_one_hour();
        assert_eq!(feb_end_plus_one.minute(), 56, "feb + min");
        assert_eq!(feb_end_plus_one.hour(), 0, "feb + h");
        assert_eq!(feb_end_plus_one.day(), 1, "feb + d");
        assert_eq!(feb_end_plus_one.month(), 3, "feb + m");
        assert_eq!(feb_end_minus_one.minute(), 56, "feb - min");
        assert_eq!(feb_end_minus_one.hour(), 22, "feb - h");
        assert_eq!(feb_end_minus_one.day(), 28, "feb - d");
        assert_eq!(feb_end_minus_one.month(), 2, "feb - m");

        //leap
        let feb_leap_end = NaiveDateTime::from_str("2020-02-29T23:56:04").unwrap();
        let feb_leap_end_plus_one = feb_leap_end.plus_one_hour();
        let feb_leap_end_minus_one = feb_leap_end.minus_one_hour();
        assert_eq!(feb_leap_end_plus_one.minute(), 56, "leap feb + min");
        assert_eq!(feb_leap_end_plus_one.hour(), 0, "leap feb + h");
        assert_eq!(feb_leap_end_plus_one.day(), 1, "leap feb + d");
        assert_eq!(feb_leap_end_plus_one.month(), 3, "leap feb + m");
        assert_eq!(feb_leap_end_minus_one.minute(), 56, "leap feb - min");
        assert_eq!(feb_leap_end_minus_one.hour(), 22, "leap feb - h");
        assert_eq!(feb_leap_end_minus_one.day(), 29, "leap feb - d");
        assert_eq!(feb_leap_end_minus_one.month(), 2, "leap feb - m");
    }




    #[test]
    fn test_one_month() {
        let august = NaiveDateTime::from_str("2015-08-18T13:56:04").unwrap();
        let september = august.plus_one_month();
        let july = august.minus_one_month();
        assert_eq!(september.minute(), 56, "+ min");
        assert_eq!(september.hour(), 13, "+ h");
        assert_eq!(september.day(), 1, "+ d");
        assert_eq!(september.month(), 9, "+ m");
        assert_eq!(july.minute(), 56, "- min");
        assert_eq!(july.hour(), 13, "- h");
        assert_eq!(july.day(), 1, "- d");
        assert_eq!(july.month(), 7, "- m");
    }
}