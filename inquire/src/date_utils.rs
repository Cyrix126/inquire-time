use core::panic;
use derive_more::Display;
use std::str::FromStr;
use time::{error::Parse, macros::format_description, Date, Month, OffsetDateTime};
pub fn get_current_date() -> Date {
    OffsetDateTime::now_local().unwrap().date()
}

pub fn get_start_date(month: Month, year: i32) -> Date {
    time::Date::from_calendar_date(year, month, 1).unwrap()
}

pub fn get_month(month: u32) -> Month {
    match month {
        1 => time::Month::January,
        2 => time::Month::February,
        3 => time::Month::March,
        4 => time::Month::April,
        5 => time::Month::May,
        6 => time::Month::June,
        7 => time::Month::July,
        8 => time::Month::August,
        9 => time::Month::September,
        10 => time::Month::October,
        11 => time::Month::November,
        12 => time::Month::December,
        _ => panic!("Invalid month"),
    }
}
pub fn get_number_month(month: Month) -> u32 {
    match month {
        time::Month::January => 1,
        time::Month::February => 2,
        time::Month::March => 3,
        time::Month::April => 4,
        time::Month::May => 5,
        time::Month::June => 6,
        time::Month::July => 7,
        time::Month::August => 8,
        time::Month::September => 9,
        time::Month::October => 10,
        time::Month::November => 11,
        time::Month::December => 12,
    }
}
pub fn display_month_fr<'a>(month: Month) -> &'a str {
    match month {
        time::Month::January => "JANVIER",
        time::Month::February => "FÉVRIER",
        time::Month::March => "MARS",
        time::Month::April => "AVRIL",
        time::Month::May => "MAI",
        time::Month::June => "JUIN",
        time::Month::July => "JUILLET",
        time::Month::August => "Août",
        time::Month::September => "SEPTEMBRE",
        time::Month::October => "OCTOBRE",
        time::Month::November => "NOVEMBRE",
        time::Month::December => "DÉCEMBRE",
    }
}
/// type for using with CustomType because the type time::Date doesn't implement FromStr
/// The type DateFromStr has the same implementation of FromStr like NaiveDate of chrono.
#[derive(Clone, Debug, Display)]
pub struct DateFromStr {
    /// the filed date contain the time::Date value
    pub date: Date,
}

impl FromStr for DateFromStr {
    type Err = Parse;

    fn from_str(s: &str) -> Result<DateFromStr, Parse> {
        let format = format_description!(version = 2, "[day]/[month]/[year]");
        match Date::parse(s, &format) {
            Ok(date) => Ok(DateFromStr { date }),
            Err(err) => Err(err),
        }
    }
}
