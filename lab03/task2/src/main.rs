fn main() {
    println!("Hello, world!");
}

fn number(bus_stops:&[(i32,i32)]) -> i32 {
    bus_stops.iter().fold(0, |a, (b, c)| a + b - c)
}

#[test]
fn returns_expected1() {
  assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
}

#[test]
fn returns_expected2() {
  assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
}

#[test]
fn returns_expected3() {
  assert_eq!(number(&[(3,0)]), 3);
}

#[test]
fn returns_expected4() {
  assert_eq!(number(&[(3,3)]), 0);
}

#[test]
fn returns_expected5() {
  assert_eq!(number(&[(3,0),(0,3)]), 0);
}

#[test]
fn returns_expected6() {
  assert_eq!(number(&[(0,0)]), 0);
}

#[test]
fn returns_expected7() {
  assert_eq!(number(&[]), 0);
}

#[test]
fn returns_expected8() {
  assert_eq!(number(&[(1, 1)]), 0);
}

#[test]
fn returns_expected9() {
  assert_eq!(number(&[(10, 0)]), 10);
}

#[test]
fn returns_expected10() {
  assert_eq!(number(&[(5, 4)]), 1);
}