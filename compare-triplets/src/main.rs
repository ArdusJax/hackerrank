use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut results = vec![0,0];
    for x in 0..3 {
        match a[x] {
            number if number > b[x] => results[0] += 1,
            number if number < b[x] => results[1] += 1,
            _ => continue,
        }
    }

    results
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}

mod test {
    use crate::compareTriplets;

    #[test]
    fn triplets_success() {
        let a = vec![10, 3, 100];
        let b = vec![10, 3, 100];
        assert_eq!(compareTriplets(&a, &b), vec![0,0]);
    }
    #[test]
    fn triplets_a_wins() {
        let a = vec![12, 3, 100];
        let b = vec![10, 3, 100];
        assert_eq!(compareTriplets(&a, &b), vec![1,0]);
    }
}