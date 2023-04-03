#[allow(non_snake_case)]
fn plusMinus(arr: &[i32]) {
    let positive: Vec<&i32> = arr.iter().filter(|i| **i > 0).collect();
    let negative: Vec<&i32> = arr.iter().filter(|i| **i < 0).collect();
    let zero: Vec<&i32> = arr.iter().filter(|i| **i == 0).collect();

    println!("positive: {:?}", positive);
    println!("negative: {:?}", negative);
    println!("zero: {:?}", zero);

    let total = arr.len() as f32;
    let positive_proportions = positive.len() as f32 / total;
    let negative_proportions = negative.len() as f32 / total;
    let zero_proportions = zero.len() as f32 / total;

    println!("positive proportions: {:.6}", positive_proportions);
    println!("negative_proportions: {:.6}", negative_proportions);
    println!("zero proportions: {:.6}", zero_proportions);
}

fn main() {
    let data = vec![0, -3, 5, 1, -29];
    plusMinus(&data);
}
