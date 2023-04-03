#[allow(non_snake_case)]
fn simpleArraySum(ar: &[i32]) -> i32 {
    ar.iter().fold(0, |mut acc, item| {
        acc += *item;
        acc
    })
}

fn main() {
    let ar = vec![4, 6, 9, 20, 84];
    println!("{}", simpleArraySum(&ar));
}
