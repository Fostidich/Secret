mod errors {
    pub mod exiting;
    pub mod codes;
}

mod functions {
    pub mod help;
    pub mod get;
    pub mod list;
}

use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};
use functions::get::scrt_get;
use functions::help::scrt_help;
use functions::list::{scrt_list_add, scrt_list_remove, scrt_list_show};
use crate::errors::codes::{DIR_NOT_FOUND, FILE_FAILURE, INVALID_ARGUMENTS, IO_ERROR, NO_COMMAND_PROVIDED, NO_DATA_FOUND, UNKNOWN_COMMAND};
use crate::errors::exiting::{Catch, end};

/// The constants contains the number of pops to be made from the executable to reach the program root folder.
const PATH_POPS: u8 = 3;

/// Main function checks for input, branching onto requested function.
///
/// # Errors
///
/// Execution stops if arguments don't follow any command standard.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        end(NO_COMMAND_PROVIDED)
    } else if args[1] == "help" {
        if args.len() != 2 {
            end(INVALID_ARGUMENTS)
        }
        scrt_help();
    } else if args[1] == "get" {
        if args.len() != 5 {
            end(INVALID_ARGUMENTS)
        }
        scrt_get(args[2].chars().collect(), args[3].chars().collect(), args[4].chars().collect());
    } else if args[1] == "list" {
        if args.len() != 5 && args.len() != 3 {
            end(INVALID_ARGUMENTS)
        }
        if args[2] == "add" {
            scrt_list_add(args[3].chars().collect(), args[4].chars().collect())
        } else if args[2] == "remove" {
            scrt_list_remove(args[3].chars().collect(), args[4].chars().collect())
        } else if args[2] == "show" {
            scrt_list_show()
        } else {
            end(UNKNOWN_COMMAND)
        }
    } else {
        end(UNKNOWN_COMMAND)
    }
}

/// Returns the file at the given path.
/// If the file is not present, it will be created,
/// along with all the necessary directories that are absent.
/// The file is in read-write mode.
///
/// # Errors
///q
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