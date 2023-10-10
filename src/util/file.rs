use std::fs::{File, OpenOptions};
use std::{env, fs};
use std::io::{Seek, Write};
use crate::PATH_POPS;
use crate::util::err_codes::{DIR_NOT_FOUND, FILE_FAILURE, IO_ERROR, NO_DATA_FOUND};
use crate::util::exiting::{Catch, end};

/// Returns the file at the given path.
/// If the file is not present, it will be created,
/// along with all the necessary directories that are absent.
/// The file is in read-write mode.
///
/// # Errors
///
/// This function may stop execution due to a wide range of cases,
/// covering many possible unrecoverable errors derived
/// from operating with files.
///
/// # Examples
///
/// ```simple_usage
/// let mut file = open_file("res/list.json");
/// let mut buff = String::new();
/// file.read_to_string(&mut buff).expect("unable to read");
/// ```
pub fn open_file(path: &str) -> File {
    let mut current_dir = env::current_exe().catch(DIR_NOT_FOUND);
    for _i in 0..PATH_POPS {
        if !current_dir.pop() {
            end(DIR_NOT_FOUND)
        }
    }
    current_dir.push(path);
    if let Some(parent) = current_dir.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).catch(FILE_FAILURE)
        }
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(false)
        .create(true)
        .open(&current_dir)
        .catch(FILE_FAILURE);
    let metadata = fs::metadata(&current_dir).catch(NO_DATA_FOUND);
    if metadata.len() == 0 {
        file.write_all(b"[]").catch(IO_ERROR);
        file.rewind().catch(FILE_FAILURE)
    }
    file
}
