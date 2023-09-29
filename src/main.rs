use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("USAGE: scrt [website] [username] [key]");
        return
    }
    if args[3].len() != 8 {
        println!("ERROR: wrong key size!");
        return
    }
    let website : Vec<char> = args[1].chars().collect();
    let username : Vec<char> = args[2].chars().collect();
    let key : Vec<char> = args[3].chars().collect();

    let mut first_block : Vec<u8> = get_block(&website, &key);
    let mut second_block : Vec<u8> = get_block(&username, &key);

    println!("{:?}\n{:?}", first_block, second_block);
    println!();

    for ch in key {
        //swap_char(&mut first_block, &mut second_block, &ch);
        //shift_rows(&mut first_block, &mut second_block, &ch);
        //shift_columns(&mut first_block, &mut second_block, &ch);
        combine_key(&mut first_block, &mut second_block, &key, &ch);

        println!("{:?}\n{:?}", first_block, second_block);
        println!();
    }


    
}

fn get_value(ch: &char) -> u8 {
    return match ch {
        'A'..='Z' => *ch as u8 - 29,
        'a'..='z' => *ch as u8 - 87,
        '0'..='9' => *ch as u8 - 48,
        '.' => 62,
        _ => 63
    }
}

fn get_block(chars: &[char], key: &[char]) -> Vec<u8> {
    let mut result : Vec<u8> = vec![0; 8];
    let mut i : usize = 0;
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
        result[i] = result[i] % 64
    }
    return result
}

fn swap_char(first_block: &mut Vec<u8>, second_block: &mut Vec<u8>, ch: &char) {
    let idx : usize = get_value(ch) as usize % 4;
    let tmp : u8 = first_block[idx];
    first_block[idx] = first_block[idx + 4];
    first_block[idx + 4] = second_block[idx + 4];
    second_block[idx + 4] = second_block[idx];
    second_block[idx] = tmp;
}

fn shift_rows(first_block: &mut Vec<u8>, second_block: &mut Vec<u8>, ch: &char) {
    let mut offset : u8 = get_value(ch) % 8;
    if get_value(ch) % 2 == 0 {
        for _i in 0..offset {
            left_shift(first_block)
        }
        if offset < 4 {
            offset += 4
        } else {
            offset += 4
        }
        for _i in 0..offset {
            right_shift(second_block)
        }
    } else {
        for _i in 0..offset {
            left_shift(second_block)
        }
        if offset < 4 {
            offset += 4
        } else {
            offset += 4
        }
        for _i in 0..offset {
            right_shift(first_block)
        }
    }
}

fn left_shift(block: &mut Vec<u8>) {
    let idx : usize = block.len()-1;
    let tmp : u8 = block[0];
    for i in 1..block.len() {
        block[i-1] = block[i]
    }
    block[idx] = tmp
}

fn right_shift(block: &mut Vec<u8>) {
    let idx : usize = block.len()-1;
    let tmp : u8 = block[idx];
    for i in (0..block.len()-1).rev() {
        block[i+1] = block[i]
    }
    block[0] = tmp
}

fn shift_columns(first_block: &mut Vec<u8>, second_block: &mut Vec<u8>, ch: &char) {
    let mut idx: usize = get_value(ch) as usize % 8;
    let quantity : u8 = get_value(ch) / 8 + 1;
    let mut tmp : u8;
    for _i in 0..quantity {
        tmp = first_block[idx];
        first_block[idx] = second_block[idx];
        second_block[idx] = tmp;
        if idx != 7 {
            idx += 1
        } else {
            idx = 0
        }
    }
}

fn combine_key(first_block: &mut Vec<u8>, second_block: &mut Vec<u8>, key: &[char], ch: &char) {
    let mut result : Vec<u8> = vec![0; 8];
    let res_offset : usize = get_value(ch) as usize % 8;
    let mut first_idx : usize = get_value(ch) as usize / 8;
    let mut second_idx : usize = get_value(ch) as usize % 8;
    for i in 0..8 {
        result[i] = (first_block[first_idx] + second_block[second_idx] + get_value(&key[i])) % 64;
        if first_idx != 7 {
            first_idx += 1
        } else {
            first_idx = 0
        }
        if second_idx != 7 {
            second_idx += 1
        } else {
            second_idx = 0
        }
    }
    if res_offset % 2 == 0 {
        for i in 0..8 {
            if i < res_offset {
                first_block[i] = result[i]
            } else {
                second_block[i] = result[i]
            }
        }
    } else {
        for i in 0..8 {
            if i < res_offset {
                second_block[i] = result[i]
            } else {
                first_block[i] = result[i]
            }
        }
    }
}