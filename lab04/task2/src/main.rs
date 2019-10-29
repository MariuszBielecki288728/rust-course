fn main() {
    println!("Hello, world!");
}

fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
  let only_even: Vec<i32> = array.into_iter().map(|n| *n).filter(|n| n % 2 == 0).collect();
  only_even[(only_even.len() - number)..only_even.len()].to_vec()
}

#[test]
fn sample_tests1() {
  assert_eq!(even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3), vec!(4, 6, 8));
}

#[test]
fn sample_tests2() {
  assert_eq!(even_numbers(&vec!(6, -25, 3, 7, 5, 5, 7, -3, 23), 1), vec!(6));
}

#[test]
fn sample_tests3() {
  assert_eq!(even_numbers(&vec!(-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26), 2), vec!(-8, 26));
}

#[test]
fn sample_tests4() {
   assert_eq!(even_numbers(&vec!(2, 2, 2), 2), vec!(2, 2));
}

#[test]
fn sample_tests5() {
  assert_eq!(even_numbers(&vec!(-1), 0), vec!());
}

#[test]
fn sample_tests6() {
  assert_eq!(even_numbers(&vec!(4, 4), 2), vec!(4, 4));
}

#[test]
fn sample_tests7() {
  assert_eq!(even_numbers(&vec!(4, 2, 1, 3, 5), 2), vec!(4, 2));
}

#[test]
fn sample_tests8() {
  assert_eq!(even_numbers(&vec!(1, 1, 8), 1), vec!(8));
}

#[test]
fn sample_tests9() {
  assert_eq!(even_numbers(&vec!(2, 1), 1), vec!(2));
}

#[test]
fn sample_tests10() {
  assert_eq!(even_numbers(&vec!(-2, 4), 2), vec!(-2, 4));
}