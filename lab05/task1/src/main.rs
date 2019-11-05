fn main() {
    println!("Hello, world!");
}

fn xo(string: &'static str) -> bool {
  string.to_lowercase().chars().filter(|&x| x == 'x').count()
  == string.to_lowercase().chars().filter(|&x| x == 'o').count()
}

#[test]
fn returns_expected1() {
  assert_eq!(xo("xo"), true);
}

#[test]
fn returns_expected2() {
  assert_eq!(xo("Xo"), true);
}

#[test]
fn returns_expected3() {
  assert_eq!(xo("xxOo"), true);
}

#[test]
fn returns_expected4() {
  assert_eq!(xo("xxxm"), false);
}

#[test]
fn returns_expected5() {
  assert_eq!(xo("Oo"), false);
}

#[test]
fn returns_expected6() {
  assert_eq!(xo("ooom"), false);
}

#[test]
fn returns_expected7() {
  assert_eq!(xo("xxxxasdfgfdbe"), false);
}

#[test]
fn returns_expected8() {
  assert_eq!(xo("xoxoxoxo"), true);
}

#[test]
fn returns_expected9() {
  assert_eq!(xo("umu"), true);
}

#[test]
fn returns_expected10() {
  assert_eq!(xo("o, xD"), true);
}