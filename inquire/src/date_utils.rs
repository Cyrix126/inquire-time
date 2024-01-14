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

#[cfg(feature = "chrono")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_date() {
        let current_date = get_current_date();
        let expected_date = chrono::Local::now().date_naive();
        assert_eq!(current_date, expected_date);
    }

    #[test]
    fn test_get_start_date() {
        assert_eq!(
            get_start_date(chrono::Month::January, 2021),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
        );
        assert_eq!(
            get_start_date(chrono::Month::February, 2021),
            chrono::NaiveDate::from_ymd_opt(2021, 2, 1).unwrap()
        );
        assert_eq!(
            get_start_date(chrono::Month::March, 2021),
            chrono::NaiveDate::from_ymd_opt(2021, 3, 1).unwrap()
        );
        assert_eq!(
            get_start_date(chrono::Month::December, 1883),
            chrono::NaiveDate::from_ymd_opt(1883, 12, 1).unwrap()
        );
        assert_eq!(
            get_start_date(chrono::Month::June, 3042),
            chrono::NaiveDate::from_ymd_opt(3042, 6, 1).unwrap()
        );
    }

    #[test]
    // this is basically a reimplementation but it works as a sanity check
    fn test_get_month() {
        assert_eq!(get_month(1), chrono::Month::January);
        assert_eq!(get_month(2), chrono::Month::February);
        assert_eq!(get_month(3), chrono::Month::March);
        assert_eq!(get_month(4), chrono::Month::April);
        assert_eq!(get_month(5), chrono::Month::May);
        assert_eq!(get_month(6), chrono::Month::June);
        assert_eq!(get_month(7), chrono::Month::July);
        assert_eq!(get_month(8), chrono::Month::August);
        assert_eq!(get_month(9), chrono::Month::September);
        assert_eq!(get_month(10), chrono::Month::October);
        assert_eq!(get_month(11), chrono::Month::November);
        assert_eq!(get_month(12), chrono::Month::December);
    }

    #[test]
    #[should_panic]
    fn test_get_month_0_panics() {
        get_month(0);
    }

    #[test]
    #[should_panic]
    fn test_get_month_13_panics() {
        get_month(13);
    }
}
