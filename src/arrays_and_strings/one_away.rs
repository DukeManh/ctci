/*
   One Away: There are three types of edits that can be performed
   on strings: insert a character, remove a character, or replace a
   character. Given two strings, write a function to check if they are
   one edit (or zero edits) away.
*/

pub fn one_away(word: String, other_word: String) -> bool {
    if i32::abs(word.len() as i32 - other_word.len() as i32) > 1 {
        return false;
    }

    let mut diff_found = false;

    let (mut i, mut j) = (word.chars(), other_word.chars());

    let (mut n1, mut n2) = (i.next(), j.next());
    while let (Some(c1), Some(c2)) = (n1, n2) {
        if c1 != c2 {
            if diff_found {
                return false;
            }
            diff_found = true;
            if word.len() == other_word.len() {
                n1 = i.next();
                n2 = j.next();
            } else if word.len() > other_word.len() {
                n1 = i.next();
            } else {
                n2 = j.next();
            }
        } else {
            n1 = i.next();
            n2 = j.next();
        }
    }

    true
}

#[test]
fn test_if_a_string_is_one_edit_from_another() {
    assert!(one_away(String::from("pale"), String::from("ple")));
    assert!(one_away(String::from("pales"), String::from("pale")));
    assert!(one_away(String::from("pale"), String::from("bale")));
    assert!(!one_away(String::from("pale"), String::from("bake")));
    assert!(one_away(String::from("p"), String::from("")));
    assert!(one_away(String::from("p"), String::from("e")));
    assert!(one_away(String::from(""), String::from("")));
    assert!(!one_away(String::from("blue"), String::from("bl")));
}
