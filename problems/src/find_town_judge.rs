// https://leetcode.com/problems/find-the-town-judge/
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut is_judge = vec![true; n as usize];
    for element in &trust {
        println!("{:?} {} {}", element, element[0] - 1, is_judge[(element[0] - 1) as usize]);
        is_judge[(element[0] - 1) as usize] = false;
    }
    println!("{:?}", is_judge);

    for person in 0..n {
        if is_judge[person as usize] {
            return person + 1;
        }
    }
    
    -1
}

pub fn main() {
    // println!("{}", find_judge(3, vec![vec![1,3],vec![2,3]]));
    assert_eq!(find_judge(3, vec![vec![1,3],vec![2,3]]), 3);
}