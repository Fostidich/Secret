use std::io::{Read, Seek, SeekFrom, Write};
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use crate::errors::codes::{FILE_FAILURE, IO_ERROR, SERDE_ERROR};
use crate::errors::exiting::Catch;
use crate::open_file;

/// The struct represent the information of a login entry.
/// Specifically, the date ([Date]) of the creation, the website name and the username used.
/// Note that two entries can have same website but must have different usernames.
#[derive(Serialize, Deserialize)]
struct Entry {
    date: Date,
    website: String,
    username: String,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.website == other.website && self.username == other.username
    }
}

impl Entry {
    /// Entry is converted in a string with format "\[date] website (username)".
    fn to_string(&self) -> String {
        format!("{} {} ({})", self.date.to_string(), self.website, self.username)
    }
}

/// The struct represent a simple date, with day, month and year.
#[derive(Serialize, Deserialize, PartialEq, Eq)]
struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    /// Date is converted in a string with format "\[day-month-year]".
    fn to_string(&self) -> String {
        format!("[{}, {}, {}]", self.day, self.month, self.year)
    }
}

/// Given entry is added to the saved list.
/// Duplicates will be ignored.
///
/// # Errors
///
/// Execution stops in case of failure while trying to read or write the content of the file.
///
/// (See also [Entry])
pub fn scrt_list_add(website: String, username: String) {
    let date_stamp = Local::now();
    let entry = Entry {
        date: Date {
            day: date_stamp.day() as u8,
            month: date_stamp.month() as u8,
            year: date_stamp.year() as u16,
        },
        website,
        username,
    };
    let mut file = open_file("res/list.json");
    let mut buff = String::new();
    file.read_to_string(&mut buff).catch(IO_ERROR);
    let mut list: Vec<Entry> = serde_json::from_str(&buff).catch(SERDE_ERROR);
    if list.contains(&entry) {
        println!("Entry already present.");
        return
    }
    list.push(entry);
    file.seek(SeekFrom::Start(0)).catch(FILE_FAILURE);
    let serialized = serde_json::to_string_pretty(&list).catch(SERDE_ERROR);
    file.write_all(serialized.as_bytes()).catch(IO_ERROR);
    println!("Entry added.")
}

/// Given entry is removed from the saved list.
/// If entry is absent, it will be ignored.
///
/// # Errors
///
/// Execution stops in case of failure while trying to read or write the content of the file.
///
/// (See also [Entry])
pub fn scrt_list_remove(website: String, username: String) {
    let date_stamp = Local::now();
    let entry = Entry {
        date: Date {
            day: date_stamp.day() as u8,
            month: date_stamp.month() as u8,
            year: date_stamp.year() as u16,
        },
        website,
        username,
    };
    let mut file = open_file("res/list.json");
    let mut buff = String::new();
    file.read_to_string(&mut buff).catch(IO_ERROR);
    let mut list: Vec<Entry> = serde_json::from_str(&buff).catch(SERDE_ERROR);
    for i in 0..list.len() {
        if list[i].eq(&entry) {
            list.remove(i);
            file.seek(SeekFrom::Start(0)).catch(FILE_FAILURE);
            let serialized = serde_json::to_string_pretty(&list).catch(SERDE_ERROR);
            file.write_all(serialized.as_bytes()).catch(IO_ERROR);
            file.set_len(serialized.len() as u64).catch(IO_ERROR);
            println!("Entry removed.");
            return
        }
    }
    println!("Entry not present.")
}

/// The list of the saved entries is printed.
///
/// # Errors
///
/// Execution stops in case of failure while trying to read the content of the file.
///
/// (See also [Entry])
pub fn scrt_list_show() {
    let mut file = open_file("res/list.json");
    let mut buff = String::new();
    file.read_to_string(&mut buff).catch(IO_ERROR);
    let list: Vec<Entry> = serde_json::from_str(&buff).catch(SERDE_ERROR);
    if list.is_empty() {
        println!("No entries to show.");
        return;
    }
    for entry in list {
        println!("{}", entry.to_string());
    }
}