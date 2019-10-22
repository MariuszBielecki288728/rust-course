fn main() {
    println!("Hello, world!");
}

fn count_bits(n: i64) -> u32 {
  n.count_ones()
}

#[test]
fn returns_expected1() {
    assert_eq!(count_bits(0), 0);
}

#[test]
fn returns_expected2() {
    assert_eq!(count_bits(1), 1);
}

#[test]
fn returns_expected3() {
    assert_eq!(count_bits(2), 1);
}

#[test]
fn returns_expected4() {
    assert_eq!(count_bits(3), 2);
}

#[test]
fn returns_expected5() {
    assert_eq!(count_bits(4), 1);
}

#[test]
fn returns_expected6() {
    assert_eq!(count_bits(5), 2);
}

#[test]
fn returns_expected7() {
    assert_eq!(count_bits(6), 2);
}

#[test]
fn returns_expected8() {
    assert_eq!(count_bits(7), 3);
}

#[test]
fn returns_expected9() {
    assert_eq!(count_bits(8), 1);
}

#[test]
fn returns_expected10() {
    assert_eq!(count_bits(9), 2);
}