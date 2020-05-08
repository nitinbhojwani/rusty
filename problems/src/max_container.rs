// https://leetcode.com/problems/container-with-most-water
use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut pointer1: usize = 0;
    let mut pointer2: usize = height.len() - 1;
    
    let mut max_area: i32 = 0;
    while pointer1 < pointer2 {
        max_area = cmp::max(max_area, (pointer2 - pointer1) as i32 * cmp::min(height[pointer1], height[pointer2]));
        if height[pointer1] < height[pointer2] {
            pointer1 += 1;
        } else {
            pointer2 -= 1;
        }
        // println!("{} {} {}", pointer1, pointer2, max_area);
    }
    return max_area;
}

pub fn main() {
    let height: Vec<i32> = vec![1,8,6,2,5,4,8,3,7];
    assert_eq!(max_area(height), 49);
}