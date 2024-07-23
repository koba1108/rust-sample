use chrono::{Datelike, NaiveDate, Utc};

pub fn is_before_birthday(birthday: &NaiveDate) -> bool {
    let now = Utc::now();
    now.month() < birthday.month() || (now.month() == birthday.month() && now.day() < birthday.day())
}

pub fn calc_age(birthday: &NaiveDate) -> i32 {
    let now = Utc::now();
    let age = now.year() - birthday.year();
    if is_before_birthday(birthday) {
        age - 1
    } else {
        age
    }
}
