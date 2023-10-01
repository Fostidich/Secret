use std::fs::File;
use std::io::Read;
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use crate::get_dir;

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
    let mut file = File::open(get_dir("res/list.json")).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let mut list : Vec<Entry> = serde_json::from_str(&buff).unwrap();
    list.push(entry);

    //TODO: dump entries back on json file
}

pub fn scrt_list_remove(website: String, username: String) {
    //TODO: remove from json file
}

pub fn scrt_list_show() {
    let mut file = File::open(get_dir("res/list.json")).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let list : Vec<Entry> = serde_json::from_str(&buff).unwrap();
    for entry in list {
        println!("{}", entry.to_string());
    }
}