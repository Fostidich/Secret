use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("USAGE: scrt [website] [username] [key]");
        return
    }
    let website : Vec<char> = args[1].chars().collect();
    let username : Vec<char> = args[2].chars().collect();
    let key : Vec<char> = args[3].chars().collect();

    for ch in website {
        println!("{}", get_value(ch));
    }

}

fn get_value(ch: char) -> i16 {
    return match ch {
        'A'..='Z' => ch as i16 - 59,
        'a'..='z' => ch as i16 - 128,
        '0'..='4' => ch as i16 - 53,
        '5'..='9' => ch as i16 - 52,
        _ => 0
    }
}