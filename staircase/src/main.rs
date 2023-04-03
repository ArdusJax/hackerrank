fn staircase(n: i32) {
    for row in 0..n {
        for column in (0..n).rev() {
            if column > row {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}

fn main() {
    staircase(5);
}
