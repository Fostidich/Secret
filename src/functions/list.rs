use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Entry {
    date: String,
    website: String,
    username: String
}

#[derive(Serialize, Deserialize)]
struct Date {
    day: u8,
    month: u8,
    year: u16
}

pub fn scrt_list_add(website: String, username: String) {

}

pub fn scrt_list_remove(website: String, username: String) {

}

pub fn scrt_list_show() {

}