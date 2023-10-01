mod functions {
    pub mod help;
    pub mod get;
    pub mod list;
}

use std::env;
use std::path::PathBuf;
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

pub fn get_dir(path: &str) -> PathBuf {
    //TODO: create file if not present
    let mut current_dir = env::current_exe().expect("ERROR: Failed to get current directory!");
    current_dir.pop();
    current_dir.pop();
    current_dir.pop();
    current_dir.push(path);
    return current_dir
}