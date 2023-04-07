#[allow(non_snake_case)]
fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let sams_house = (s..=t).collect::<Vec<_>>();
    println!("Range = {:#?}", &sams_house);
    let num_apples = apples
        .into_iter()
        .filter(|x| sams_house.contains(&(a + *x)));
    let num_oranges = oranges
        .into_iter()
        .filter(|o| sams_house.contains(&(b + *o)));
    println!("{}", num_apples.count());
    println!("{}", num_oranges.count());
}

fn main() {
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];
    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}
