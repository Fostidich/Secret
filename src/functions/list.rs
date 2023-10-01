use chrono::{Datelike, Local};

struct Entry {
    date: Date,
    website: String,
    username: String,
}

struct Date {
    day: u8,
    month: u8,
    year: u16
}

pub fn scrt_list_add(website: String, username: String) {
    let date_stamp = Local::now();
    let entry = Entry {
        date: Date {
            day: date_stamp.day() as u8,
            month: date_stamp.month() as u8,
            year: date_stamp.year() as u16
        },
        website,
        username,
    };
}

pub fn scrt_list_remove(website: String, username: String) {}

pub fn scrt_list_show() {}