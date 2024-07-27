use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Time {
    pub second: i32,
    pub minute: i32,
    pub hour: i32,
    pub day_of_week: i32,
    pub day_of_month: i32,
    pub day_of_year: i32,
    pub month: i32,
    pub year: i32,
}

impl Time {
    fn from_chrono<T: TimeZone>(dt: DateTime<T>) -> Self {
        let time = dt.time();
        let date = dt.date_naive();

        Self {
            second: time.second() as i32,
            minute: time.minute() as i32,
            hour: time.hour() as i32,
            day_of_week: date.weekday().num_days_from_sunday() as i32,
            day_of_month: date.day() as i32,
            day_of_year: date.ordinal() as i32,
            month: date.month() as i32,
            year: date.year() as i32,
        }
    }
}

#[luajit_ffi_gen::luajit_ffi(clone = true, opaque = false)]
impl Time {
    pub fn get_local() -> Self {
        Time::from_chrono(Local::now())
    }

    pub fn get_utc() -> Self {
        Time::from_chrono(Utc::now())
    }

    pub fn get_raw() -> u32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32
    }
}
