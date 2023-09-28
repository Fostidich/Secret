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

    let mut first_block : Vec<u8> = get_block(&website, &key);
    let mut second_block : Vec<u8> = get_block(&username, &key);

    for ch in key {
        swap_char(&mut first_block, &mut second_block, &ch);
        shift_rows(&mut first_block, &mut second_block, &ch);
        shift_columns(&mut first_block, &mut second_block, &ch);
        combine_key(&mut first_block, &mut second_block, &ch);
    }

    
}

fn get_value(ch: &char) -> u8 {
    return match ch {
        'A'..='Z' => ch as u8 - 29,
        'a'..='z' => ch as u8 - 87,
        '0'..='9' => ch as u8 - 48,
        '.' => 62,
        _ => 63,
    }
}

fn get_block(chars: &[char], key: &[char]) -> Vec<u8> {
    let result : Vec<u8> = vec![0; 8];
    let mut i : u8 = 0;
    for j in 1..8 {
        if get_value(&key[j]) < 32 {
            i += 1
        }
    }
    for ch in chars {
        result[i] += get_value(ch);
        if i != 7 {
            i += 1
        } else {
            i = 0
        }
    }
    for i in 0..8 {
        result[i] = result[i]%64
    }
    return result;
}

fn swap_char(first_block: &mut Vec<u8>, second_block: &mut Vec<u8>, ch: &char) {

}

fn shift_rows(first_block: &mut Vec<u8>, second_block: &mut Vec<u8>, ch: &char) {

}

fn shift_columns(first_block: &mut Vec<u8>, second_block: &mut Vec<u8>, ch: &char) {

}

fn combine_key(first_block: &mut Vec<u8>, second_block: &mut Vec<u8>, ch: &char) {

}