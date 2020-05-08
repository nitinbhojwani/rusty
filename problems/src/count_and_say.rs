// https://leetcode.com/problems/count-and-say/
/// The count-and-say sequence is the sequence of integers with the first five terms as following:
/// 1.     1
/// 2.     11
/// 3.     21
/// 4.     1211
/// 5.     111221
/// 1 is read off as "one 1" or 11.
/// 11 is read off as "two 1s" or 21.
/// 21 is read off as "one 2, then one 1" or 1211.
/// 
/// Given an integer n where 1 ≤ n ≤ 30, generate the nth term of the count-and-say sequence. 
/// You can do so recursively, in other words from the previous member read off the digits, counting the number of digits in groups of the same digit.
/// Note: Each term of the sequence of integers will be represented as a string.
 
use std::str::FromStr;

pub fn count_and_say(n: i32) -> String {
    let mut res = String::from_str("1").to_owned().unwrap();
    let mut curr: String;
    let mut prev: char;
    let mut count: u8 = 0;

    for _i in 1..n {
        curr = res.clone();
        prev = res.chars().nth(0).unwrap();
        res.clear();

        for chr in curr.chars() {
            if prev != chr {
                res += &count.to_string();
                res.push(prev);
                count = 1;
            } else {
                count += 1;
            }
            prev = chr;
        }
        
        // res += &count.to_string();
        // res.push(prev);
    }

    return res;
}

pub fn main() {
    assert_eq!("1", count_and_say(1));
    assert_eq!("11", count_and_say(2));
    assert_eq!("21", count_and_say(3));
    assert_eq!("1211", count_and_say(4));
    assert_eq!("111221", count_and_say(5));
}