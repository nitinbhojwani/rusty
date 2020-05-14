// https://leetcode.com/problems/implement-trie-prefix-tree/
use std::option::Option;

struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie{
            children: Default::default(),
            is_end: false
        }
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut curr_node = self;
        let mut diff: usize;
        let mut chars = word.chars().peekable();
        let mut ch: char;
        while chars.peek().is_some() {
            ch = chars.next().unwrap();
            diff = ch as usize - 'a' as usize;
            if !curr_node.children[diff].is_some() {
                curr_node.children[diff] = Some(Box::new(Trie::new()));
            }
            curr_node = curr_node.children[diff].as_mut().unwrap().as_mut();
            if !chars.peek().is_some() {
                curr_node.is_end = true;
            }
        }
    }
    
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut curr_node = self;
        let mut chars = word.chars().peekable();
        let mut diff: usize;
        let mut ch: char;
        while chars.peek().is_some() {
            ch = chars.next().unwrap();
            diff = ch as usize - 'a' as usize;
            if !curr_node.children[diff].is_some() {
                return false;
            }
            curr_node = curr_node.children[diff].as_ref().unwrap().as_ref();
            if !chars.peek().is_some() {
                return curr_node.is_end;
            }
        }
        
        false
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut curr_node = self;
        let mut chars = prefix.chars().peekable();
        let mut diff: usize;

        let mut ch: char;
        while chars.peek().is_some() {
            ch = chars.next().unwrap();
            diff = ch as usize - 'a' as usize;
            if !curr_node.children[diff].is_some() {
                return false;
            }
            curr_node = curr_node.children[diff].as_ref().unwrap().as_ref();
        }
        true
    }
}


pub fn main() {
    let word = String::from("apple");
    let mut obj = Trie::new();
    obj.insert(word);
    assert_eq!(obj.search("apple".to_string()), true);
    assert_eq!(obj.search("app".to_string()), false);
    assert_eq!(obj.search("b".to_string()), false);
    assert_eq!(obj.search("apples".to_string()), false);

    assert_eq!(obj.starts_with("a".to_string()), true);
    assert_eq!(obj.starts_with("apple".to_string()), true);
    assert_eq!(obj.starts_with("apples".to_string()), false);
    assert_eq!(obj.starts_with("b".to_string()), false);
}
 