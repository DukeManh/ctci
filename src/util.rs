use std::cmp::Ordering;

pub fn compare_slice<T: Ord + std::fmt::Debug>(a: &[T], b: &[T]) -> Ordering {
    for (i, j) in a.iter().zip(b.iter()) {
        match (*i).cmp(&*j) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }

    return a.len().cmp(&b.len());
}

pub fn build_char_table(word: String) -> [usize; 96] {
    let mut count = [0usize; 128 - 32];
    for c in word.chars().into_iter() {
        let char_index = c as usize - 32;
        count[char_index] += 1;
    }
    count
}

pub fn get_char_number(c: char) -> usize {
    let char_num = c as usize;
    return if 'a' as usize <= char_num && char_num <= 'z' as usize {
        char_num - 'a' as usize
    } else if 'A' as usize <= char_num && char_num <= 'Z' as usize {
        char_num - 'A' as usize
    } else {
        0
    };
}

// Assume an English alphabet, using i32, with 32 bits to represent the vector
pub fn build_bit_vector(phrase: String) -> i32 {
    let mut bit_vector = 0;

    for c in phrase.chars().into_iter() {
        let index = get_char_number(c);
        toggle(index, &mut bit_vector);
    }

    bit_vector
}

fn toggle(index: usize, bit_vector: &mut i32) {
    let mask = 1 << index;

    // Toggle off the bit is already flipped, on otherwise
    if *bit_vector & mask == 0 {
        // if index hasn't been flipped: 00100 & 00010 = 0
        *bit_vector |= mask; // then 00100 | 00010 = 00110
    } else {
        // if index has been flipped: 00100 & 00100 != 0
        *bit_vector &= !mask; // then 'bitwise not' mask = 11101, 00100 & 11011 = 00000
    }
}
