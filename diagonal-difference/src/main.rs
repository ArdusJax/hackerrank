use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut primary = 0;
    let mut secondary = 0;
    arr.into_iter().enumerate().for_each(|(index, row)| {
        let tail = arr.len() - ( index + 1 );
        primary += row[index];
        secondary += row[tail];
    });
    let result = primary - secondary;
    result.abs()
}

fn main() {
    let data = vec![vec![11,2,4], vec![4,5,6], vec![10,8,-12]];
    let answer = diagonalDifference(&data);
    println!("The answer is {answer}");
}

mod test {
    use super::*;

    #[test]
    fn diagonal_difference_succeeds() {
        let data = vec![vec![0,3,4], vec![1,5,7], vec![9,11,13]];
        assert_eq!(18, diagonalDifference(&data));
    }
}