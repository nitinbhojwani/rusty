// https://leetcode.com/problems/course-schedule/
struct Solution {}

impl Solution {
    pub fn trav(i: usize, graph: &Vec<Vec<bool>>, mut visited: &mut Vec<bool>, mut all_visited: &mut Vec<bool>) -> bool {
        // println!("{:?}", &all_visited);
        visited[i] = true;
        for j in 0..visited.len() {
            if i != j && graph[i][j] {
                if all_visited[j] {
                    continue;
                }
                if visited[j] || !Solution::trav(j, &graph, &mut visited, &mut all_visited){
                    return false;
                }
            }
        }
        all_visited[i] = true;
        visited[i] = false;

        true
    }

    pub fn can_finish(n: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = n as usize;
        let mut graph: Vec<Vec<bool>> = vec![vec![false; num_courses]; num_courses];
        // println!("{:?}", &graph);

        for prerequisite in prerequisites {
            // println!("{} {}", prerequisite[0], prerequisite[1]);
            if prerequisite[1] == prerequisite[0] {
                return false;
            }
            graph[prerequisite[0] as usize][prerequisite[0] as usize] = true;
            graph[prerequisite[1] as usize][prerequisite[0] as usize] = true;    
        }
        
        let mut all_visited = vec![false; num_courses];
        let mut visited;
        for i in 0..num_courses {
            if !graph[i][i] { //an acyclic graph should start at i
                visited = vec![false; num_courses];
                if !(Solution::trav(i, &graph, &mut visited, &mut all_visited)) {
                    return false;
                }
            }
            // println!("{:?}", &all_visited);
        }
        
        for i in 0..num_courses {
            if !all_visited[i] {
                return false;
            }
        }
        
        true
    }
}

pub fn main() {
    assert_eq!(Solution::can_finish(5, vec![vec![1,2], vec![2,3], vec![3,4]]), true);
    assert_eq!(Solution::can_finish(5, vec![vec![1,2], vec![2,3], vec![3,4], vec![0,2]]), true);
    assert_eq!(Solution::can_finish(4, vec![vec![1,2], vec![2,3], vec![3,1]]), false);
    assert_eq!(Solution::can_finish(4, vec![vec![1,2], vec![2,3], vec![3,2]]), false);
    assert_eq!(Solution::can_finish(3, vec![vec![1,2], vec![0,1], vec![0,2]]), true);
    assert_eq!(Solution::can_finish(3, vec![vec![1,0], vec![1,2], vec![0,1]]), false);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}