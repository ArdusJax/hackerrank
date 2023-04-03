use std::collections::HashMap;

fn count_the_words(text: &str) -> HashMap<&str, i32> {
    let words = text.split_whitespace().collect::<Vec<&str>>();
    let word_counts: HashMap<&str, i32> = HashMap::new();
    words.iter().fold(word_counts, |mut acc, word| {
        acc.entry(word).and_modify(|count| *count += 1).or_insert(1);
        acc
    })
}
fn main() {
    let the_words = "There are some who would say that this is a lot of crap. Are we ever going to use this in real life? Who would say that we need this?";
    let totals = count_the_words(&the_words);
    totals.into_iter().for_each(|(k, v)| {
        println!("Word: {k} Count: {v}");
    })
}
