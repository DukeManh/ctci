/*
   String Compression: Implement a method to perform basic string compression using the counts
   of repeated characters. For example, the string aabcccccaaa would become a2b1c5a3. If the
   "compressed" string would not become smaller than the original string, your method should return
   the original string. You can assume the string has only uppercase and lowercase letters (a - z).
*/
use std::time::Instant;

fn compress(str: String) -> String {
    // There is no need to use a string builder as Strings in Rust are already resizable Vectors of char
    let mut compressed = String::from("");
    let mut count_consecutive = 0;

    let now = Instant::now();
    let chars = str.chars().into_iter().collect::<Vec<char>>();
    for i in 0..chars.len() {
        count_consecutive += 1;

        if i + 1 >= chars.len() || chars[i + 1] != chars[i] {
            compressed += &format!("{}{}", &chars[i].to_string(), count_consecutive.to_string());
            count_consecutive = 0;
        }
    }
    println!("{}", now.elapsed().as_nanos());
    if compressed.len() < str.len() {
        compressed
    } else {
        str
    }
}

#[test]
fn test_basic_string_compression() {
    assert_eq!(compress(String::from("aabcccccaaa")), "a2b1c5a3");
    assert_eq!(compress(String::from("abc")), "abc");
    assert_eq!(compress(String::from("333444422188888888")), "3344221188");
}
