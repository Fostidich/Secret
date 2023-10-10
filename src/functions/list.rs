use std::{fmt, fs, io};
use std::fmt::Formatter;
use std::io::{Read, Seek, SeekFrom, Write};
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use crate::util::err_codes::{FILE_FAILURE, IO_ERROR, SERDE_ERROR, UNKNOWN_ERROR};
use crate::util::exiting::Catch;
use crate::util::json::get_from_json;
use crate::util::file::open_file;

/// Constant stores the default path for here used file.
const LIST_PATH: &str = "scrt-data/list.json";

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

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ({})", self.date, self.website, self.username)
    }
}

/// The struct represent a simple date, with day, month and year.
#[derive(Serialize, Deserialize, PartialEq, Eq)]
struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:02}, {:02}, {}]", self.day, self.month, self.year)
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
    let mut file = open_file(LIST_PATH);
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
    let mut file = open_file(LIST_PATH);
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
    let list: Vec<Entry> = get_from_json::<Vec<Entry>>(LIST_PATH);
    if list.is_empty() {
        println!("No entries to show.");
        return;
    }
    for entry in list {
        println!("{}", entry);
    }
}

/// All the entries of the list are removed.
/// A message asking to confirm pops up before deletion.
pub fn scrt_list_destroy() {
    print!("Do you really want to destroy the list of entries (write CONFIRM to continue)? ");
    io::stdout().flush().catch(UNKNOWN_ERROR);
    let mut input = String::new();
    io::stdin().read_line(&mut input).catch(UNKNOWN_ERROR);
    if !input.trim().to_string().eq("CONFIRM") {
        println!("Nothing was destroyed.");
        return
    }
    match fs::remove_file(LIST_PATH) {
        Ok(_) => println!("List file destroyed."),
        Err(_) => println!("Nothing to destroy.")
    }
}

/// After a get, if the entry is not present in the list, user is asked if he
/// wants to add it.
pub fn check_addition(website: String, username: String) {
    let date_stamp = Local::now();
    let entry: Entry = Entry {
        date: Date {
            day: date_stamp.day() as u8,
            month: date_stamp.month() as u8,
            year: date_stamp.year() as u16,
        },
        website: website.clone(),
        username: username.clone(),
    };
    let list: Vec<Entry> = get_from_json::<Vec<Entry>>(LIST_PATH);
    if list.contains(&entry) {
        return
    }
    print!("Do you want to add entry to the list (Y/n)? ");
    io::stdout().flush().catch(UNKNOWN_ERROR);
    let mut input = String::new();
    io::stdin().read_line(&mut input).catch(UNKNOWN_ERROR);
    if !input.starts_with("Y") && !input.starts_with("y") {
        return
    }
    scrt_list_add(website, username)
}
