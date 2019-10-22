fn main() {
    println!("Hello, world!");
}

fn get_count(string: &str) -> usize {
  string.chars().filter(
      |ch| ['a', 'e', 'i', 'o', 'u'].contains(ch)).count()
}


#[test]
fn my_test1() {
  assert_eq!(get_count("abracadabra"), 5);
}

#[test]
fn my_test2() {
  assert_eq!(get_count("aeuio"), 5);
}

#[test]
fn my_test3() {
  assert_eq!(get_count(""), 0);
}

#[test]
fn my_test4() {
  assert_eq!(get_count("bcd"), 0);
}

#[test]
fn my_test5() {
  assert_eq!(get_count("aba"), 2);
}

#[test]
fn my_test6() {
  assert_eq!(get_count("aaaaaaaaaa"), 10);
}

#[test]
fn my_test7() {
  assert_eq!(get_count("bbbbbbba"), 1);
}

#[test]
fn my_test8() {
  assert_eq!(get_count("123"), 0);
}

#[test]
fn my_test9() {
  assert_eq!(get_count("puciuwa"), 4);
}

#[test]
fn my_test10() {
  assert_eq!(get_count("bbbbbbba"), 1);
}