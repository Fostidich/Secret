/// The help function prints all the possible usages of the program.
pub fn scrt_help() {
    println!("USAGE: use \"scrt\" followed by function name and respective arguments");
    println!("\tscrt get [website] [username] [key]");
    println!("\tscrt list show");
    println!("\tscrt list add [website] [username]");
    println!("\tscrt list remove [website] [username]");
    println!("\tscrt list renew [website] [username]");
    println!("\tscrt list reset [website] [username]");
    println!("\tscrt list destroy");
    println!("\tscrt help");
}