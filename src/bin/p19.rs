#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum WeekDate {
    Mon,
    Tues,
    Wed,
    Thurs,
    Fri,
    Sat,
    Sun,
}
fn next_week_date(week_day: WeekDate) -> WeekDate {
    use WeekDate::*;
    match week_day {
        Mon => Tues,
        Tues => Wed,
        Wed => Thurs,
        Thurs => Fri,
        Fri => Sat,
        Sat => Sun,
        Sun => Mon
    }
}

type Day = u64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}
fn next_month(month: Month) -> Month {
    use Month::*;
    match month {
        Jan => Feb,
        Feb => Mar,
        Mar => Apr,
        Apr => May,
        May => Jun,
        Jun => Jul,
        Jul => Aug,
        Aug => Sep,
        Sep => Oct,
        Oct => Nov,
        Nov => Dec,
        Dec => Jan,
    }
}

type Year = u64;

type Date = (WeekDate, Day, Month, Year);

fn is_leap_year(year: Year) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

fn days_in_month(month: Month, year: Year) -> u64 {
    use Month::*;
    match month {
        Apr | Jun | Sep | Nov => 30,
        Jan | Mar | May | Jul | Aug | Oct | Dec => 31,
        Feb => {
            if is_leap_year(year) {29}
            else {28}
        }
    }
}

fn iterate_date(date: Date) -> Date {
    let (week_date, day, month, year) = date;
    let nwd = next_week_date(week_date);
    if day == days_in_month(month, year) {
        if month == Month::Dec {
            (nwd, 1, Month::Jan, year + 1)
        }
        else {
            (nwd, 1, next_month(month), year)
        }
    }
    else {
        (nwd, day + 1, month, year)
    }
}

fn solve() -> u64 {
    let mut d: Date = (WeekDate::Mon, 1, Month::Jan, 1900);
    std::iter::from_fn(move || {
        d = iterate_date(d);
       // println!("next day: {:?}", d);
        match d {
            (_, _, _, 2001) => None,
            _ => Some(d)
        }
    })
    .filter(|&(wd, d, _, y)| wd == WeekDate::Sun && d == 1 && y > 1900)
    .fold(0, |acc, _| acc + 1)
}

fn main() -> () {
    let result: u64 = solve();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterate_date_test() {
        use super::iterate_date as id;
        use super::WeekDate::*;
        use super::Month::*;
        assert_eq!(id((Sun, 31, Dec, 1998)), (Mon, 1, Jan, 1999));
        assert_eq!(id((Wed, 5, Mar, 1956)), (Thurs, 6, Mar, 1956));
        assert_eq!(id((Fri, 28, Feb, 1996)), (Sat, 29, Feb, 1996));
        assert_eq!(id((Fri, 28, Feb, 1997)), (Sat, 1, Mar, 1997));
        assert_eq!(id((Fri, 28, Feb, 2000)), (Sat, 29, Feb, 2000));
    }
}
