fn sorted_squared_array(array: Vec<i32>) -> Vec<i32> {
    let mut a = array.into_iter().fold(Vec::new(), |mut acc, item| {
        acc.push(item * item);
        acc
    });
    a.sort();
    a
}

fn sorted_squared_array_raw(array: Vec<i32>) -> Vec<i32> {
    let mut start = 0;
    let mut end = array.len() - 1;
    let mut squares = Vec::new();
    squares.resize(array.len(), 0);

    for i in (0..=end).rev() {
        if array[start] * -1 > array[end] {
            squares[i] = array[start] * array[start];
            start += 1;
        } else {
            squares[i] = array[end] * array[end];
            end -= 1;
        }
    }

    squares
}

fn main() {
    let test = vec![-5, -3, 1, 2, 4];
    let answer = sorted_squared_array(test.clone());
    let raw_answer = sorted_squared_array_raw(test);
    println!("Answer with build in sort: {:?}", answer);
    println!("Answer without build in sort: {:?}", raw_answer);
}
