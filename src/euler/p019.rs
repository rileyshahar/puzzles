//! How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

#[derive(Debug, PartialEq)]
enum Month {
    Janurary,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    const fn days(&self, leap: bool) -> u8 {
        match self {
            Janurary | March | May | July | August | October | December => 31,
            February => {
                if leap {
                    29
                } else {
                    28
                }
            }
            April | June | September | November => 30,
        }
    }

    const fn next(self) -> Self {
        match self {
            Janurary => February,
            February => March,
            March => April,
            April => May,
            May => June,
            June => July,
            July => August,
            August => September,
            September => October,
            October => November,
            November => December,
            December => Janurary,
        }
    }
}

use Month::{
    April, August, December, February, Janurary, July, June, March, May, November, October,
    September,
};

#[derive(Debug, PartialEq)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    const fn next(self) -> Self {
        match self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }
    }
}

use Day::{Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday};

const fn is_leap(year: u32) -> bool {
    // every fourth year is a leap year, except centuries not divisible by 400
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    year % 4 == 0
}

#[derive(Debug)]
struct Date {
    month: Month,
    date: u8,
    year: u32,
    day: Option<Day>, // we don't always care abt the day of the week, ex for equality checks
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.month == other.month && self.date == other.date && self.year == other.year
    }
}

impl Date {
    const START_OF_1900: Self = Self {
        month: Janurary,
        date: 1,
        year: 1900,
        day: Some(Monday),
    };

    const START_OF_1901: Self = Self {
        month: Janurary,
        date: 1,
        year: 1901,
        day: None,
    };

    const START_OF_2001: Self = Self {
        month: Janurary,
        date: 1,
        year: 2001,
        day: None,
    };

    #[cfg(test)]
    const START_OF_1902: Self = Self {
        month: Janurary,
        date: 1,
        year: 1902,
        day: None,
    };

    fn next(mut self) -> Self {
        let leap = is_leap(self.year);

        self.day = self.day.map(Day::next);
        self.date = self.date % self.month.days(leap) + 1; // add one after mod so we can 1-index days
        if self.date == 1 {
            self.month = self.month.next();
            if self.month == Janurary {
                self.year += 1;
            }
        }

        self
    }
}

// until is not inclusive
fn solve_for(until: &Date) -> u32 {
    let mut date = Date::START_OF_1900;
    let mut counts = 0;
    let mut started = false;
    while &date != until {
        if date == Date::START_OF_1901 {
            started = true;
        }
        if started && date.date == 1 && date.day == Some(Sunday) {
            counts += 1;
        }
        date = date.next();
    }
    counts
}

super::example!(&Date::START_OF_1902 => 2);
super::problem!(u32: &Date::START_OF_2001 => 171);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_leap_works() {
        assert!(is_leap(2000));
        assert!(is_leap(1944));
        assert!(!is_leap(1900));
        assert!(!is_leap(1954));
    }

    #[allow(clippy::redundant_pub_crate)] // definite false positive, rust-lang/rust-clippy #8756
    macro_rules! date_next_test {
        ($name:ident: $din:literal $min:expr, $yin:expr => $dout:literal $mout:expr, $yout:expr ) => {
            #[test]
            fn $name() {
                let date_in = Date {
                    month: $min,
                    date: $din,
                    year: $yin,
                    day: None,
                };

                let date_out = Date {
                    month: $mout,
                    date: $dout,
                    year: $yout,
                    day: None,
                };

                assert!(date_in.next() == date_out);
            }
        };
    }

    date_next_test!(normal: 1 Janurary, 1900 => 2 Janurary, 1900);
    date_next_test!(month_rollover: 31 Janurary, 1900 => 1 February, 1900);
    date_next_test!(not_leap: 28 February, 1900 => 1 March, 1900); // 1900 is not a leap year
    date_next_test!(leap: 28 February, 1936 => 29 February, 1936); // 1936 is a leap year
    date_next_test!(year_rollover: 31 December, 1936 => 1 Janurary, 1937);
}
