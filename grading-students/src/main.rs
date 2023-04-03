/// Grade Cases
/// - less than 38
/// - greater than or equal to 38 and less than 100
///   - get the difference between the next grade and the round offset
///     - check the value of the difference
///       - if the difference is greater than 2 round up
///
#[allow(non_snake_case)]
fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grades.into_iter().fold(Vec::new(), |mut acc, grade| {
        let test_grade = *grade;
        if test_grade < 38 {
            acc.push(test_grade);
        }
        if test_grade >= 38 && test_grade <= 100 {
            let diff = test_grade % 5;
            if diff > 2 {
                acc.push((test_grade - diff) + 5);
            } else {
                acc.push(test_grade)
            }
        }
        acc
    })
}

fn main() {
    let grades = vec![98, 67, 38, 33];
    let final_grades = gradingStudents(&grades);
    final_grades.iter().for_each(|grade| println!("{grade}"));
}
// ##########  Alternate way to do this using a match statement #########
//grades.into_iter().fold(Vec::new(), |mut acc, grade| {
// match grade {
//     x if *x < 38 => acc.push(*x),
//     x if *x >= 38 && *x <= 100 => {
//         let diff = x % 5;
//         if diff > 2 {
//             acc.push((x - diff) + 5);
//         } else {
//             acc.push(*x);
//         }
//     }
//     _ => println!("error computing the grade"),
// }
// acc
//})
