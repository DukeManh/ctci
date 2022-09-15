/*
   Palindrome Permutation: Given a string, write a function to check if
   it is a permutation of a palin-drome. A palindrome is a word or phrase
   that is the same forwards and backwards. A permutation is a rearrangement
   of letters. The palindrome does not need to be limited to just dictionary words
*/
use crate::util::{build_bit_vector, build_char_table};

fn is_permutation_of_palindrome(phrase: String) -> bool {
    let count = build_char_table(phrase);

    let mut odd_count_found = false;
    for c in count {
        if c % 2 == 0 {
            continue;
        } else {
            if odd_count_found {
                return false;
            } else {
                odd_count_found = true;
            }
        }
    }

    true
}

fn is_permutation_of_palindrome_one_loop(phrase: String) -> bool {
    let mut count = [0usize; 128 - 32];
    let mut odd_count = 0usize;
    for c in phrase.chars().into_iter() {
        let char_index = c as usize - 32;
        count[char_index] += 1;

        if count[char_index] % 2 == 1 {
            odd_count += 1;
        } else {
            odd_count -= 1;
        }
    }

    odd_count < 2
}

fn is_permutation_of_palindrome_bit_vec(phrase: String) -> bool {
    let bit_vector = build_bit_vector(phrase);

    // a positive bits means odd number of char
    // if there is only positive bit, say 0b100, when subtracted by 1 will equal 0b011
    // 'bitwise and' the result with the bit_vector, we'll get 0
    // 0b100
    //&0b011
    //=0b000
    (bit_vector & (bit_vector - 1)) == 0
}

#[test]
fn test_is_palindrome_permutation() {
    assert!(is_permutation_of_palindrome(String::from("taocotcoa")));

    assert!(is_permutation_of_palindrome_one_loop(String::from(
        "taocotcoa"
    )));

    assert!(is_permutation_of_palindrome_bit_vec(String::from(
        "taocotcoa"
    )));

    assert_eq!(
        is_permutation_of_palindrome("2020".into()),
        is_permutation_of_palindrome_one_loop("2002".into())
    )
}
