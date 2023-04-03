use std::collections::HashMap;

#[allow(non_snake_case)]
fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let counts: HashMap<&i32, i32> = HashMap::new();
    let result = candles.iter().fold(counts, |mut acc, candle| {
        acc.entry(candle)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        acc
    });

    result.into_iter().max().unwrap().1
}
fn main() {
    let candles = vec![3, 4, 4, 5, 5, 5, 5, 5, 1];
    println!("{}", birthdayCakeCandles(&candles));
}
