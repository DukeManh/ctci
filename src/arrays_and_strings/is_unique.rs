/*
   Is Unique: Implement an algorithm to determine if a string has
   all unique characters. What if you cannot use additional data structures?
*/
pub fn is_unique(word: String) -> bool {
    // There must be at least 2 same characters
    if word.len() > 128 {
        return false;
    }

    // Assume that words contains first 96 printable ascii characters
    let mut count = [false; 128 - 32];

    // No need to keep track of count, return false if a character has been seen
    for c in word.chars().into_iter() {
        let char_index = c as usize - 32;
        if count[char_index] {
            return false;
        }
        count[char_index] = true;
    }

    return true;
}

#[test]
fn test_word_contains_unique_characters_only() {
    let unique = is_unique(String::from("TheQuickBrown"));
    assert!(unique);

    let unique = is_unique(String::from("TheQuickBrownFox"));
    assert!(!unique);
}
