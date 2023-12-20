use core::panic;
// use derive_more::Display;
use std::{fmt, str::FromStr};
use time::{error::Parse, macros::format_description, Date, Month, OffsetDateTime};
pub fn get_current_date() -> Date {
    let datetime = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    datetime.date()
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
#[cfg(feature = "lang-fr")]
pub fn display_month_fr<'a>(month: Month) -> &'a str {
    match month {
        time::Month::January => "Janvier",
        time::Month::February => "Février",
        time::Month::March => "Mars",
        time::Month::April => "Avril",
        time::Month::May => "Mai",
        time::Month::June => "Juin",
        time::Month::July => "Juillet",
        time::Month::August => "Août",
        time::Month::September => "Septembre",
        time::Month::October => "Octobre",
        time::Month::November => "Novembre",
        time::Month::December => "Décembre",
    }
}
/// type for using with CustomType because the type time::Date doesn't implement FromStr
/// The type DateFromStr has the same implementation of FromStr like NaiveDate of chrono.
#[derive(Clone, Debug)]
pub struct DateFromStr {
    /// the filed date contain the time::Date value
    pub date: Date,
}
impl fmt::Display for DateFromStr {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.date)
    }
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
