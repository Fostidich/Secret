mod functions {
    pub mod help;
    pub mod get;
    pub mod list;
}

use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};
use std::process::exit;
use functions::help::scrt_help;
use crate::functions::get::scrt_get;
use crate::functions::list::{scrt_list_add, scrt_list_remove, scrt_list_show};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("ERROR: no command provided! Try using \"scrt help\"");
        return;
    } else if args[1] == "help" {
        if args.len() != 2 {
            println!("ERROR: invalid arguments! Try using \"scrt help\"");
            return;
        }
        scrt_help();
        return;
    } else if args[1] == "get" {
        if args.len() != 5 {
            println!("ERROR: invalid arguments! Try using \"scrt help\"");
            return;
        }
        scrt_get(args[2].chars().collect(), args[3].chars().collect(), args[4].chars().collect());
        return;
    } else if args[1] == "list" {
        if args.len() != 5 && args.len() != 3 {
            println!("ERROR: invalid arguments! Try using \"scrt help\"");
            return;
        }
        if args[2] == "add" {
            scrt_list_add(args[3].chars().collect(), args[4].chars().collect())
        } else if args[2] == "remove" {
            scrt_list_remove(args[3].chars().collect(), args[4].chars().collect())
        } else if args[2] == "show" {
            scrt_list_show()
        } else {
            println!("ERROR: unknown command! Try using \"scrt help\"")
        }
        return;
    }
    println!("ERROR: unknown command! Try using \"scrt help\"")
}

pub fn open_file(path: &str) -> File {
    let steps_opening = || -> Result<File, Box<dyn Error>> {
        let mut current_dir;
        let file;
        current_dir = env::current_exe()?;
        for _i in 0..3 {
            if !current_dir.pop() {
                println!("ERROR: failed to retrieve path!");
                exit(1)
            }
        }
        current_dir.push(path);
        file = File::open(current_dir.clone())?;
        Ok(file)
    };
    let steps_creating = || -> Result<File, Box<dyn Error>> {
        let mut current_dir;
        let mut file;
        current_dir = env::current_exe()?;
        for _i in 0..3 {
            if !current_dir.pop() {
                println!("ERROR: failed to retrieve path!");
                exit(1)
            }
        }
        current_dir.push(path);
        file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(current_dir.clone())?;
        write!(&mut file, "[]")?;
        file.rewind()?;
        Ok(file)
    };
    match steps_opening() {
        Err(_) => {
            match steps_creating() {
                Err(_) => {
                    println!("ERROR: failed to open/create file!");
                    exit(1)}
                Ok(file) => {file}
            }
        }
        Ok(file) => {file}
    }
}