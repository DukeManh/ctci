/*
   Check Permutation: Given two strings, write a method to decide if one
   is a permutation of the other
*/

use crate::util::{build_char_table, compare_slice};

use std::cmp::Ordering;

pub fn is_permutation(word: String, other_word: String) -> bool {
    // Think of what cases in which we can tell the answer immediately
    if word.len() != other_word.len() {
        return false;
    };

    // Solution 1: Count occurrences of each character in each string and compare
    // Solution 2: Use a bit vector
    let word_chars = build_char_table(word);
    let other_word_chars = build_char_table(other_word);
    return compare_slice(&word_chars, &other_word_chars) == Ordering::Equal;
}

#[test]
fn test_string_permutation() {
    let permutation = is_permutation(String::from("hello"), String::from("llohe"));
    println!("{}", permutation);
    assert!(permutation);

    let permutation = is_permutation(String::from("hello"), String::from("hallo"));
    assert!(!permutation);
}
