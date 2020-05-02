// https://leetcode.com/problems/string-compression/

use std::char;

fn convert_to_char(count: &u32) -> Vec<char> {
    let mut num = count.clone();
    let mut char_arr = Vec::<char>::new();
    while num > 0 {
        char_arr.push(char::from_digit(num % 10, 10).unwrap());
        num /= 10;
    }
    char_arr.reverse();
    
    return char_arr;
} 

pub fn compress(chars: &mut Vec<char>) -> i32 {
    if chars.len() == 0 {
        return 0;
    }

    let mut prev = chars[0];
    let mut count = 1;
    let mut res = vec![chars[0]];

    for i in 1..chars.len() {
        if chars[i] == prev || count == 0 {
            count += 1
        } else {
            if count > 1 {
                res.append(&mut convert_to_char(&count));
            }
            count = 1;
            res.push(chars[i]);
        } 
        prev = chars[i];
    }
    if count > 1 {
        res.append(&mut convert_to_char(&count));
    }
    chars.clear();
    chars.append(&mut res);
    return chars.len() as i32;
}

pub fn main() {
    let count = 10;
    assert_eq!(vec!['1', '0'], convert_to_char(&count));
    // println!("{:?}", convert_to_char(&count));

    let mut chars = vec!['a', 'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'c'];
    assert_eq!(6, compress(&mut chars));
    assert_eq!(chars, ['a', '2', 'b', '1', '2', 'c']);
    // println!("{:?}", chars);
}