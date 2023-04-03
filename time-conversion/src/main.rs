// Time complexity of O(n)
// The loop over the chars is what makes this O(n) as we have to loop
// over each char in the string
#[allow(non_snake_case)]
fn timeConversion(s: &str) -> String {
    let is_pm = s.ends_with("PM");
    let time_chunks = s
        .trim_matches(|c| c == 'A' || c == 'M' || c == 'P')
        .split(':')
        .collect::<Vec<&str>>();

    let hour = time_chunks[0].parse::<i32>().unwrap();
    if hour == 12 && !is_pm {
        return String::from(format!(
            "{:2}:{:2}:{:2}",
            "00", &time_chunks[1], &time_chunks[2]
        ));
    }
    if is_pm && hour != 12 {
        return format!(
            "{:2}:{:2}:{:2}",
            hour + 12,
            &time_chunks[1],
            &time_chunks[2]
        );
    }
    format!(
        "{:0>2}:{:2}:{:2}",
        &time_chunks[0], &time_chunks[1], &time_chunks[2]
    )
}

// Time complexity of O(1) as it is effectively working
// on a stack like structure (Vec) and there are no loops
#[allow(non_snake_case)]
fn timeConversionConstant(s: &str) -> String {
    let is_pm = s.ends_with("PM");
    let hour = s.get(0..2).unwrap().parse::<i32>().unwrap();
    let minutes = s.get(3..5).unwrap();
    let seconds = s.get(6..8).unwrap();
    if hour == 12 && !is_pm {
        return String::from(format!("{:2}:{:2}:{:2}", "00", &minutes, &seconds));
    }
    if is_pm && hour != 12 {
        return format!("{:2}:{:2}:{:2}", hour + 12, &minutes, &seconds);
    }
    format!("{:0>2}:{:2}:{:2}", &hour, &minutes, &seconds)
}

fn main() {
    let time = "01:05:45AM";
    println!("0(n) time: {}", timeConversion(&time));
    println!("0(1) time: {}", timeConversionConstant(&time));
}
