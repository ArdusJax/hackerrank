/*
 * Complete the 'kangaroo' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER x1
 *  2. INTEGER v1
 *  3. INTEGER x2
 *  4. INTEGER v2
 */

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    let mut count = 0;
    let answer = String::from("YES");

    if x1 == x2 {
        return answer;
    }

    while count < 10000 {
        if x1 + v1 * count == x2 + v2 * count {
            return answer;
        }
        count += 1;
    }

    String::from("NO")
}

fn kangaroo_simple(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    let mut x1 = x1;
    let mut x2 = x2;
    let mut count = 0;

    while count < 10000 {
        if x1 == x2 {
            return String::from("YES");
        }

        x1 += v1;
        x2 += v2;
        count += 1;
    }

    String::from("NO")
}

fn main() {
    let answer = kangaroo(0, 2, 5, 3);
    println!("{answer}");
}
