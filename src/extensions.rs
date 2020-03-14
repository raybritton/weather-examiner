use chrono::{NaiveDateTime, Duration, Timelike, Datelike};
use std::ops::{Add, Sub};
use crate::models::SimpleDate;

pub trait Utils {
    fn minus_one_hour(&self) -> Self;
    fn plus_one_hour(&self) -> Self;
    fn minus_one_day(&self) -> Self;
    fn plus_one_day(&self) -> Self;
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

pub trait MapToUnit <E> {
    fn map_to_unit(self) -> Result<(), E>;
}

impl <U,E> MapToUnit<E> for Result<U, E> {
    fn map_to_unit(self) -> Result<(), E> {
        return self.map(|_| unit());
    }
}