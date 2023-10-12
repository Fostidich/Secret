use crate::functions::list::{check_addition, Entry};
use crate::LIST_PATH;
use crate::util::err_codes::INVALID_ARGUMENTS;
use crate::util::exiting::end;
use crate::util::json::get_from_json;

/// The get function prints the hash calculated as a combination of the website, username and key strings.
/// The strategy used distantly resembles the AES approach.
/// Format used is "abCD-1234", granting 4 letters (2 of which randomly uppercase) and 4 digits.
///
/// # Errors
///
/// Execution stops if the key doesn't have the correct length.
pub fn scrt_get(website: Vec<char>, username: Vec<char>, mut key: Vec<char>) {
    if key.len() != 8 {
        end(INVALID_ARGUMENTS)
    }
    check_renewed(&website, &username, &mut key);
    let mut first_block: Vec<u8> = get_block(&website, &key);
    let mut second_block: Vec<u8> = get_block(&username, &key);
    let key_ref = &key;
    for ch in key_ref {
        swap_char(&mut first_block, &mut second_block, ch);
        shift_rows(&mut first_block, &mut second_block, ch);
        shift_columns(&mut first_block, &mut second_block, ch);
        combine_key(&mut first_block, &mut second_block, key_ref, ch);
    }
    let mut result_nums: Vec<u8> = vec![0; 8];
    for i in 0..8 {
        result_nums[i] = (first_block[i] + second_block[i]) / 2
    }
    let mut result: Vec<char> = vec!['-'; 9];
    for i in 0..4 {
        result[i] = (result_nums[i] % 26 + 97) as char
    }
    for i in 4..8 {
        result[i + 1] = (result_nums[i] % 10 + 48) as char
    }
    put_uppercase(&mut result, key_ref);
    let to_print: String = result.into_iter().collect();
    println!("{}", to_print);
    check_addition(website.into_iter().collect(), username.into_iter().collect())
}

/// Given a char, it returns a number between 0 and 63.
fn get_value(ch: &char) -> u8 {
    match ch {
        'A'..='Z' => *ch as u8 - 29,
        'a'..='z' => *ch as u8 - 87,
        '0'..='9' => *ch as u8 - 48,
        '.' => 62,
        _ => 63
    }
}

/// Given a number, it returns a letter or a digit.
fn get_char(val: &u8) -> char {
    match val {
        36..=61 => (val + 29) as char,
        10..=35 => (val + 87) as char,
        0..=9 => (val + 48) as char,
        62 => '.',
        _ => '-'
    }
}

/// Given a string and a key, a vector of numbers is returned based on input.
fn get_block(chars: &[char], key: &[char]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; 8];
    let mut i: usize = 0;
    for ch in key {
        if get_value(ch) < 32 {
            i += 1
        }
    }
    if let Some(n) = i.checked_sub(1) {
        i = n;
    }
    for ch in chars {
        result[i] += get_value(ch);
        if i != 7 {
            i += 1
        } else {
            i = 0
        }
    }
    for i in result.iter_mut() {
        *i %= 64;
    }
    result
}

/// Given two blocks and a char, elements of the blocks are swapped.
/// The swap occurs both inside the same block and between them.
fn swap_char(first_block: &mut [u8], second_block: &mut [u8], ch: &char) {
    let idx: usize = get_value(ch) as usize % 4;
    let tmp: u8 = first_block[idx];
    first_block[idx] = first_block[idx + 4];
    first_block[idx + 4] = second_block[idx + 4];
    second_block[idx + 4] = second_block[idx];
    second_block[idx] = tmp;
}

/// Given two blocks and a char, elements of the blocks are shifted horizontally.
/// Shift quantity and direction is based on input.
fn shift_rows(first_block: &mut [u8], second_block: &mut [u8], ch: &char) {
    let mut offset: u8 = get_value(ch) % 8;
    if get_value(ch) % 2 == 0 {
        for _i in 0..offset {
            left_shift(first_block)
        }
        if offset < 4 {
            offset += 4
        } else {
            offset -= 4
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
            offset -= 4
        }
        for _i in 0..offset {
            right_shift(first_block)
        }
    }
}

/// Given a vector, elements are shifted left once.
fn left_shift(block: &mut [u8]) {
    let idx: usize = block.len() - 1;
    let tmp: u8 = block[0];
    for i in 1..block.len() {
        block[i - 1] = block[i]
    }
    block[idx] = tmp
}

/// Given a vector, elements are shifted right once.
fn right_shift(block: &mut [u8]) {
    let idx: usize = block.len() - 1;
    let tmp: u8 = block[idx];
    for i in (0..block.len() - 1).rev() {
        block[i + 1] = block[i]
    }
    block[0] = tmp
}

/// Given two blocks and a char, elements of the blocks are shifted vertically.
/// Shift location is based on input.
fn shift_columns(first_block: &mut [u8], second_block: &mut [u8], ch: &char) {
    let mut idx: usize = get_value(ch) as usize % 8;
    let quantity: u8 = get_value(ch) / 8 + 1;
    let mut tmp: u8;
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

/// Given three blocks and a char, elements of the blocks are combined together to modify the values of
/// the first two blocks.
fn combine_key(first_block: &mut [u8], second_block: &mut [u8], key: &[char], ch: &char) {
    let mut result: Vec<u8> = vec![0; 8];
    let res_offset: usize = get_value(ch) as usize % 8;
    let mut first_idx: usize = get_value(ch) as usize / 8;
    let mut second_idx: usize = get_value(ch) as usize % 8;
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

/// Given a string and a char, some letters of the string will be set to uppercase based on input.
fn put_uppercase(block: &mut [char], key: &[char]) {
    let idx: usize = (get_value(&key[7]) % 8) as usize;
    let mut os: u8 = 0;
    for ch in &mut *block {
        os += get_value(ch) / 4;
    }
    match (get_value(&key[idx]) + os) % 6 {
        0 => {
            block[3] = (block[3] as u8 - 32) as char;
            block[2] = (block[2] as u8 - 32) as char;
        }
        1 => {
            block[3] = (block[3] as u8 - 32) as char;
            block[1] = (block[1] as u8 - 32) as char;
        }
        2 => {
            block[2] = (block[2] as u8 - 32) as char;
            block[1] = (block[1] as u8 - 32) as char;
        }
        3 => {
            block[3] = (block[3] as u8 - 32) as char;
            block[0] = (block[0] as u8 - 32) as char;
        }
        4 => {
            block[2] = (block[2] as u8 - 32) as char;
            block[0] = (block[0] as u8 - 32) as char;
        }
        5 => {
            block[1] = (block[1] as u8 - 32) as char;
            block[0] = (block[0] as u8 - 32) as char;
        }
        _ => {}
    }
}

/// In case an entry has been renewed, the get function automatically get the renewed parameter
/// and modifies the entry accordingly to get the correct relative renewed hash.
fn check_renewed(website: &Vec<char>, username: &Vec<char>, key: &mut Vec<char>) {
    let entry = Entry {
        date: Default::default(),
        website: website.into_iter().collect(),
        username: username.into_iter().collect(),
        renewed: 0,
    };
    let list: Vec<Entry> = get_from_json::<Vec<Entry>>(LIST_PATH);
    if !list.contains(&entry) {
        return
    }
    let mut ren: u8 = 0;
    for el in list.iter() {
        if el == &entry {
            if el.renewed != 0 {
                ren = el.renewed;
                break
            } else {
                return
            }
        }
    }
    let last = key.len()-1;
    key[last] = get_char(&(get_value(&key[last])+ren))
}