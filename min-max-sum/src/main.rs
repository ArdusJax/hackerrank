fn mini_max_sum(arr: &[i32]) {
    let mut max = arr[0] as i64;
    let mut min = arr[0] as i64;
    let mut total = 0;

    for i in 0..arr.len() {
        if arr[i] as i64 > max {
            max = arr[i] as i64;
        }
        if (arr[i] as i64) < min {
            min = arr[i] as i64;
        }
        total = total + arr[i] as i64;
    }

    //println!("max value: {max} / max sum: {}", total - min);
    //println!("min value: {min} / min sum: {}", total - max);
    println!("{} {}", total - max, total - min);
}

fn main() {
    let a = vec![1, 2, 13, 44, 5, 33, 1, 21, 5, 19, 77, 9, 393, 1, 2, 99];
    mini_max_sum(&a);
}
