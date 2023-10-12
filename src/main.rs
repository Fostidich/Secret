mod util {
    pub mod exiting;
    pub mod err_codes;
    pub mod file;
    pub mod json;
}

mod functions {
    pub mod help;
    pub mod get;
    pub mod list;
}

use std::env;
use functions::get::scrt_get;
use functions::help::scrt_help;
use functions::list::{scrt_list_add, scrt_list_remove, scrt_list_show, scrt_list_destroy};
use util::err_codes::{INVALID_ARGUMENTS, NO_COMMAND_PROVIDED, UNKNOWN_COMMAND};
use util::exiting::end;
use crate::functions::list::{scrt_list_renew, scrt_list_reset};

/// The constants contains the number of pops to be made from the executable path to reach the program root folder.
const PATH_POPS: u8 = 1;

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
        } else if args[2] == "destroy" {
            scrt_list_destroy()
        } else if args[2] == "renew" {
            scrt_list_renew(args[3].chars().collect(), args[4].chars().collect())
        } else if args[2] == "reset" {
            scrt_list_reset(args[3].chars().collect(), args[4].chars().collect())
        } else {
            end(UNKNOWN_COMMAND)
        }
    } else {
        end(UNKNOWN_COMMAND)
    }
}