#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    pub fn plural(self) -> String {
        match self {
            TimeUnit::Seconds => String::from("seconds"),
            TimeUnit::Minutes => String::from("minutes"),
            TimeUnit::Hours => String::from("hours"),
            TimeUnit::Days => String::from("days"),
            TimeUnit::Months => String::from("months"),
            TimeUnit::Years => String::from("years"),
            _ => String::from(""),
        }
    }

    pub fn singular(self) -> String {
        self.plural().trim_end_matches('s').to_string()
    }
}

///
/// A timestamp that has been deliberately rounded off so our program
/// says "6 months ago" instead of the actual date
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

pub fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}
