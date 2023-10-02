use std::io::Read;
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use crate::open_file;

#[derive(Serialize, Deserialize)]
struct Entry {
    date: Date,
    website: String,
    username: String
}

impl Entry {
    fn to_string(&self) -> String {
        format!("{} {} ({})", self.date.to_string(), self.website, self.username)
    }
}

#[derive(Serialize, Deserialize)]
struct Date {
    day: u8,
    month: u8,
    year: u16
}

impl Date {
    fn to_string(&self) -> String {
        format!("[{}, {}, {}]", self.day, self.month, self.year)
    }
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
    let mut file = open_file("res/list.json");
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let mut list : Vec<Entry> = serde_json::from_str(&buff).unwrap();
    list.push(entry);

    //TODO: dump entries back on json file
}

pub fn scrt_list_remove(_website: String, _username: String) {
    //TODO: remove from json file
}

pub fn scrt_list_show() {
    let mut file = open_file("res/list.json");
    let mut buff = String::new();
    match file.read_to_string(&mut buff) {
        Err(_) => {
            panic!("ERROR: unable to read from file!")
        }
        Ok(_) => {}
    }
    let list : Vec<Entry>;
    match serde_json::from_str(&buff) {
        Err(_) => {
            panic!("ERROR: unable to retrieve json data from file!")
        }
        Ok(data) => {list = data}
    }
    if list.is_empty() {
        println!("No entries to show.");
        return
    }
    for entry in list {
        println!("{}", entry.to_string());
    }
}