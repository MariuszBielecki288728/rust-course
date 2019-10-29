fn main() {
    println!("Hello, world!");
}

fn find_digit(num: i32, nth: i32) -> i32 {
  if nth <= 0 {return -1}
  num.abs().to_string().chars().rev()
    .nth((nth - 1) as usize).unwrap_or('0')
    .to_digit(10).unwrap() as i32
}

#[test]
fn example_test1() {
  assert_eq!(find_digit(5673, 4), 5);
}

#[test]
fn example_test2() {
  assert_eq!(find_digit(129, 2), 2);
}

#[test]
fn example_test3() {
  assert_eq!(find_digit(-2825, 3), 8);
}

#[test]
fn example_test4() {
  assert_eq!(find_digit(-456, 4), 0);
}

#[test]
fn example_test5() {
  assert_eq!(find_digit(0, 20), 0);
}

#[test]
fn example_test6() {
  assert_eq!(find_digit(65, 0), -1);
}

#[test]
fn example_test7() {
  assert_eq!(find_digit(24, -8), -1);
}

#[test]
fn example_test8() {
  assert_eq!(find_digit(0, 1), 0);
}

#[test]
fn example_test9() {
  assert_eq!(find_digit(1, 0), -1);
}

#[test]
fn example_test10() {
  assert_eq!(find_digit(-123, 4), 0);
}