use std::collections::HashMap;
use std::env;
use std::str::FromStr;

fn fibonacci<'a>(n: &u64, memo: &'a mut HashMap<u64, u64>) -> &'a u64 {
    if !(memo.contains_key(n)) {
        println!("Calculating Fibonacci for {}", n);
        {
            fibonacci(&(*n - 1), memo);
        }
        memo.insert(
            *n,
            *memo.get(&(*n - 1)).unwrap() + *memo.get(&(*n - 2)).unwrap(),
        );
    }
    println!("Returning Fibonacci for {}", n);
    memo.get(n).unwrap()
}

pub fn main() {
    let inputs: Vec<String> = env::args().collect();
    if inputs.len() < 2 {
        eprintln!("Usage: ./fibonacci 10");
        return;
    }
    let mut memo: HashMap<u64, u64> = HashMap::new();
    memo.insert(1, 1);
    memo.insert(2, 1);
    println!("Memo: {:?}", &memo);
    println!("Memo: {} {}", memo.get(&1).unwrap(), memo.get(&2).unwrap());
    println!(
        "Fibonacci of {}: {}",
        inputs[1],
        fibonacci(&u64::from_str(&inputs[1]).unwrap(), &mut memo)
    );
    println!("Memo: {:?}", &memo);
}

#[test]
fn test_fibonacci() {
    let mut memo: HashMap<u64, u64> = HashMap::new();
    memo.insert(1, 1);
    memo.insert(2, 1);
    let asserts: Vec<u64> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    for i in 1..10 {
        println!("{}", i);
        assert_eq!(fibonacci(&i, &mut memo), asserts.get(i as usize).unwrap());
    }
}
