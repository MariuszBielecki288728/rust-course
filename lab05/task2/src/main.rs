fn main() {
    println!("Hello, world!");
}

fn descending_order(x: u64) -> u64 {
    let mut vec: Vec<char> = x.to_string().chars().collect();
    vec.sort();
    vec.reverse();
    return vec.into_iter().collect::<String>().parse::<u64>().unwrap()
}

#[test]
fn returns_expected1() {
    assert_eq!(descending_order(0), 0);
}

#[test]
fn returns_expected2() {
    assert_eq!(descending_order(1), 1);
}

#[test]
fn returns_expected3() {
    assert_eq!(descending_order(15), 51);
}

#[test]
fn returns_expected4() {
    assert_eq!(descending_order(1021), 2110);
}

#[test]
fn returns_expected5() {
    assert_eq!(descending_order(123456789), 987654321);
}

#[test]
fn returns_expected6() {
    assert_eq!(descending_order(145263), 654321);
}

#[test]
fn returns_expected7() {
    assert_eq!(descending_order(1254859723), 9875543221);
}

#[test]
fn returns_expected8() {
    assert_eq!(descending_order(123), 321);
}

#[test]
fn returns_expected9() {
    assert_eq!(descending_order(111), 111);
}

#[test]
fn returns_expected10() {
    assert_eq!(descending_order(141), 411);
}