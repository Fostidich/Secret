mod functions {
    pub mod help;
    pub mod get;
    pub mod list;
}

use std::env;
use functions::help::scrt_help;
use crate::functions::get::scrt_get;
use crate::functions::list::scrt_list;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("ERROR: no arguments! Try using \"scrt help\"");
        return
    }
    else if args[1] == "help" {
        if args.len() != 2 {
            println!("ERROR: too many arguments! Try using \"scrt help\"");
            return
        }
        scrt_help();
        return
    }
    else if args[1] == "get" {
        if args.len() != 5 {
            println!("ERROR: invalid arguments! Try using \"scrt help\"");
            return
        }
        scrt_get(args[2].chars().collect(), args[3].chars().collect(), args[4].chars().collect());
        return;
    }
    else if args[1] == "list" {
        if args.len() != 2 {
            println!("ERROR: too many arguments! Try using \"scrt help\"");
            return
        }
        scrt_list();
        return
    }
    println!("ERROR: unknown command! Try using \"scrt help\"")
}