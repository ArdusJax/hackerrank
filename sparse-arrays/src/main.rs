use std::collections::HashMap;

#[allow(non_snake_case)]
fn matchingStrings(stringList: &[String], queries: &[String]) -> Vec<i32> {
    let db = stringList
        .into_iter()
        .fold(HashMap::new(), |mut acc, item| {
            acc.entry(item).and_modify(|count| *count += 1).or_insert(1);
            acc
        });
    let answers = queries.into_iter().fold(Vec::new(), |mut acc, item| {
        if let Some(s) = db.get(item) {
            acc.push(*s);
        } else {
            acc.push(0);
        }
        acc
    });
    answers
}

fn main() {
    let stringList = vec![
        String::from("aba"),
        String::from("baba"),
        String::from("aba"),
        String::from("xzxb"),
    ];
    let queries = vec![
        String::from("aba"),
        String::from("xzxb"),
        String::from("ab"),
    ];
    let answers = matchingStrings(&stringList, &queries);
    answers.iter().for_each(|answer| println!("{answer}"));
}
