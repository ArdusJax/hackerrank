fn sorted_squared_array(array: Vec<i32>) -> Vec<i32> {
    let mut a = array.into_iter().fold(Vec::new(), |mut acc, item| {
        acc.push(item * item);
        acc
    });
    a.sort();
    a
}

fn main() {
    let test = vec![-5, -3, 1, 2, 4];
    let answer = sorted_squared_array(test);
    println!("{:?}", answer);
}
