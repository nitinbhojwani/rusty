// https://leetcode.com/problems/single-element-in-a-sorted-array/
pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    let mut l: usize = 0;
    let mut r: usize = nums.len() - 1;
    let mut mid: usize;
    while l <= r {
        mid = l + (r - l) / 2;
        
        if (mid == 0  || nums[mid] != nums[mid - 1]) && (mid == nums.len() - 1 || nums[mid] != nums[mid + 1]) {
            return nums[mid];
        } else if mid > 0 && nums[mid] == nums[mid - 1] {
            if mid % 2 == 0 {
                r = mid - 2;
            } else {
                l = mid + 1;
            }
        } else if mid + 1 < nums.len() && nums[mid] == nums[mid + 1] {
            if (mid + 1) % 2 == 0 {
                r = mid - 1;
            } else {
                l = mid + 2;
            }
        }
    }
    
    -1
}

pub fn main() {
    assert_eq!(1, single_non_duplicate(vec![1]));
    assert_eq!(5, single_non_duplicate(vec![1,1,2,2,3,3,4,4,5]));
    assert_eq!(3, single_non_duplicate(vec![1,1,2,2,3,4,4,5,5]));
    assert_eq!(-1, single_non_duplicate(vec![]));
}