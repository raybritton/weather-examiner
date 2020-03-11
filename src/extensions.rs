use chrono::{NaiveDateTime, Duration, Timelike, Datelike};
use std::ops::{Add, Sub};
use crate::models::SimpleDate;

pub trait Utils {
    fn minus_one_hour(&self) -> Self;
    fn plus_one_hour(&self) -> Self;
}

impl Utils for NaiveDateTime {
    fn minus_one_hour(&self) -> NaiveDateTime {
        return self.sub(Duration::hours(1));
    }

    fn plus_one_hour(&self) -> NaiveDateTime {
        return self.add(Duration::hours(1));
    }
}

impl From<SimpleDate> for NaiveDateTime {
    fn from(value: SimpleDate) -> Self {
        NaiveDateTime::from_timestamp(0, 0)
            .with_year(value.year as i32).expect("Bad year")
            .with_ordinal0(value.day as u32).expect("Bad day")
            .with_hour(value.hour as u32).expect("Bad hour")
    }
}