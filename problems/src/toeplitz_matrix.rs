// https://leetcode.com/problems/toeplitz-matrix/
pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let prev_i = i as i32 - 1;
            let prev_j = j as i32 - 1;
            if prev_i < 0|| prev_j < 0 || matrix[i-1][j-1] == matrix[i][j] {
                continue;
            }
            return false;
        }
    }

    true
}

pub fn main() {
    let matrix: Vec<Vec<i32>> = vec![vec![1,2,3,4], vec![5,1,2,3], vec![9,5,1,2]];
    println!("{:?}", matrix);
    assert_eq!(true, is_toeplitz_matrix(matrix));

    let matrix: Vec<Vec<i32>> = vec![vec![1,4,3,2], vec![5,1,2,3], vec![9,5,1,2]];
    println!("{:?}", matrix);
    assert_eq!(false, is_toeplitz_matrix(matrix));

}

// #[config(test)]