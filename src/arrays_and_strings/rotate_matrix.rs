/*
   Rotate Matrix: Given an image represented by an NxN matrix, where each pixel in the image is 4
   bytes, write a method to rotate the image by 90 degrees. (an you do this in place?
*/
pub fn rotate_matrix<T: Copy>(matrix: &mut Vec<Vec<T>>) -> bool {
    if matrix.len() == 0 || matrix.len() != matrix[0].len() {
        return false;
    }

    let n = matrix.len();

    for layer in 0..(n / 2 as usize) {
        let first = layer;
        let last = n - 1 - layer;

        for i in first..last {
            let offset = i - first;
            let top = matrix[first][i];
            matrix[first][i] = matrix[last - offset][first];
            matrix[last - offset][first] = matrix[last][last - offset];
            matrix[last][last - offset] = matrix[i][last];
            matrix[i][last] = top;
        }
    }

    true
}

#[test]
fn test_matrix_rotation() {
    let mut matrix_1 = vec![
        vec!['a', 'b', 'c', 'd', 'e'],
        vec!['f', 'g', 'h', 'i', 'j'],
        vec!['k', 'l', 'm', 'n', 'o'],
        vec!['p', 'q', 'r', 's', 't'],
        vec!['u', 'v', 'w', 'x', 'y'],
    ];

    assert!(rotate_matrix(&mut matrix_1));
    assert_eq!(
        matrix_1,
        [
            ['u', 'p', 'k', 'f', 'a'],
            ['v', 'q', 'l', 'g', 'b'],
            ['w', 'r', 'm', 'h', 'c'],
            ['x', 's', 'n', 'i', 'd'],
            ['y', 't', 'o', 'j', 'e']
        ]
    );

    let mut matrix_2 = vec![vec![1, 2], vec![3, 4]];
    assert!(rotate_matrix(&mut matrix_2));
    assert_eq!(matrix_2, [[3, 1], [4, 2],])
}
