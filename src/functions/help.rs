/// The help function prints all the possible usages of the program.
pub fn scrt_help() {
    let white = "\x1B[1;37m";
    let reset = "\x1B[0m";
    let arr = char::from_u32(0x27A4).unwrap_or('>');
    println!("Usage: use \"scrt\" followed by a command and the respective arguments\n");
    println!("\t{}scrt get [website] [username] {}{} calculates the hash from the entry and the asked key", white, arr, reset);
    println!("\t{}scrt list show {}{} shows the list of past used entries", white, arr, reset);
    println!("\t{}scrt list add [website] [username] {}{} adds the entry to the list", white, arr, reset);
    println!("\t{}scrt list remove [website] [username] {}{} removes the entry from the list", white, arr, reset);
    println!("\t{}scrt list renew [website] [username] {}{} in case password has to be changed, sets the entry as renewed", white, arr, reset);
    println!("\t{}scrt list reset [website] [username] {}{} resets the renewed number parameter to zero", white, arr, reset);
    println!("\t{}scrt list destroy {}{} destroys the whole entries file", white, arr, reset);
    println!("\t{}scrt help {}{} opens this very view", white, arr, reset);
    println!("\nPlease feel free to open a discussion/issue on https://github.com/Fostidich/Secret if you encounter any issues or have questions")
}