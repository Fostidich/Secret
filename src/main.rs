mod functions {
    pub mod help;
    pub mod get;
    pub mod list;
}

use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};
use crate::functions::get::scrt_get;
use crate::functions::help::scrt_help;
use crate::functions::list::{scrt_list_add, scrt_list_remove, scrt_list_show};

/// The constants contains the number of pops to be made from the executable to reach the program root folder.
const PATH_POPS: u8 = 3;

/// Main function checks for input, branching onto requested function.
///
/// # Panics
///
/// It panics if arguments don't follow any command standard.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("ERROR: no command provided! Try using \"scrt help\"")
    } else if args[1] == "help" {
        if args.len() != 2 {
            panic!("ERROR: invalid arguments! Try using \"scrt help\"")
        }
        scrt_help();
    } else if args[1] == "get" {
        if args.len() != 5 {
            panic!("ERROR: invalid arguments! Try using \"scrt help\"")
        }
        scrt_get(args[2].chars().collect(), args[3].chars().collect(), args[4].chars().collect());
    } else if args[1] == "list" {
        if args.len() != 5 && args.len() != 3 {
            panic!("ERROR: invalid arguments! Try using \"scrt help\"")
        }
        if args[2] == "add" {
            scrt_list_add(args[3].chars().collect(), args[4].chars().collect())
        } else if args[2] == "remove" {
            scrt_list_remove(args[3].chars().collect(), args[4].chars().collect())
        } else if args[2] == "show" {
            scrt_list_show()
        } else {
            panic!("ERROR: unknown command! Try using \"scrt help\"")
        }
    } else {
        panic!("ERROR: unknown command! Try using \"scrt help\"")
    }
}

/// Returns the file at the given path.
/// If the file is not present, it will be created,
/// along with all the necessary directories that are absent.
/// The file is in read-write mode.
///
/// # Panics
///
/// This function may panic in a wide range of cases,
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
    let mut current_dir = env::current_exe().expect("ERROR: failed to get current directory!");
    for _i in 0..PATH_POPS {
        if !current_dir.pop() {
            panic!("ERROR: failed to retrieve path!")
        }
    }
    current_dir.push(path);
    if let Some(parent) = current_dir.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("ERROR: failed to create directories!");
        }
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(false)
        .create(true)
        .open(&current_dir)
        .expect("ERROR: failed to open or create file!");
    let metadata = fs::metadata(&current_dir).expect("ERROR: failed to get file metadata!");
    if metadata.len() == 0 {
        file.write_all(b"[]").expect("ERROR: failed to initialize file\nWARNING: file may be corrupted!");
        file.rewind().expect("ERROR: failed to navigate into file!")
    }
    file
}