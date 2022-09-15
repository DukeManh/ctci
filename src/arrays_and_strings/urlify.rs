/*
   URLify: Write a method to replace all spaces in a string with '%20:
   You may assume that the string has sufficient space at the end to
   hold the additional characters, and that you are given the "true"
   length of the string. (Note: If implementing in Java, please use a
   character array so that you can perform this operation in place.)
*/

pub fn url_ify(chars: &mut [char], true_length: usize) {
    // Jo_nh_sm____, true_length = 8
    // 012345678901
    // last = 8 + num_spaces * 2 = 12
    // for i from 7 -> 0
    //    if is_space: s[last] = 0, s[last - 1] = 2, s[last - 2] = %
    //    if is_char: s[last] = s[i]

    let mut num_spaces = 0;

    for i in 0..true_length {
        if chars[i] == ' ' {
            num_spaces += 1;
        }
    }

    let mut last = num_spaces * 2 + true_length - 1;

    for i in (0..true_length).rev() {
        println!("{:?}", chars);
        if chars[i] != ' ' {
            chars[last] = chars[i];
            if last >= 1 {
                last -= 1;
            }
        } else {
            chars[last] = '0';
            chars[last - 1] = '2';
            chars[last - 2] = '%';
            if last >= 3 {
                last -= 3;
            }
        }
    }
}

#[test]
fn test_url_ify() {
    let phrase = "Mr John Smith JR      ";
    let result = "Mr%20John%20Smith%20JR";
    let mut url = [' '; 22];
    let mut escaped_url = [' '; 22];
    for (i, c) in phrase.chars().enumerate() {
        url[i] = c;
    }
    for (i, c) in result.chars().enumerate() {
        escaped_url[i] = c;
    }

    url_ify(&mut url, 16);
    assert_eq!(url, escaped_url);
}
