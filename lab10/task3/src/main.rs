fn main() {
    println!("Hello, world!");
}

use std::cmp::Ordering;

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_worth_tab = vec![1, 2, 3, 3, 4, 10];
    let evil_worth_tab = vec![1, 2, 2, 2, 3, 5, 10];
    let good_worth = calculate_worth(good_worth_tab, good);
    let evil_worth = calculate_worth(evil_worth_tab, evil);

    String::from(match good_worth.cmp(&evil_worth) {
        Ordering::Less => "Battle Result: Evil eradicates all trace of Good",
        Ordering::Equal => "Battle Result: No victor on this battle field",
        Ordering::Greater => "Battle Result: Good triumphs over Evil",
    })
}

fn calculate_worth(worth_tab: Vec<i32>, numbers: &str) -> i32 {
    worth_tab
        .iter()
        .zip(
            numbers
                .split_whitespace()
                .map(|d| d.parse::<i32>().unwrap()),
        )
        .fold(0, |acc, (worth, numbers)| acc + worth * numbers)
}

#[test]
fn returns_expected1() {
    assert_eq!(
        good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"),
        "Battle Result: No victor on this battle field"
    );
}

#[test]
fn returns_expected2() {
    assert_eq!(
        good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"),
        "Battle Result: Good triumphs over Evil"
    );
}

#[test]
fn returns_expected3() {
    assert_eq!(
        good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"),
        "Battle Result: Evil eradicates all trace of Good"
    );
}

#[test]
fn returns_expected4() {
    assert_eq!(
        good_vs_evil("0 0 0 0 0 4", "0 0 0 0 0 0 10"),
        "Battle Result: Evil eradicates all trace of Good"
    );
}

#[test]
fn returns_expected5() {
    assert_eq!(
        good_vs_evil("1 0 0 0 0 10", "0 0 1 0 0 0 10"),
        "Battle Result: Evil eradicates all trace of Good"
    );
}

#[test]
fn returns_expected6() {
    assert_eq!(
        good_vs_evil("0 1 0 0 0 10", "0 0 1 0 0 0 10"),
        "Battle Result: No victor on this battle field"
    );
}

#[test]
fn returns_expected7() {
    assert_eq!(
        good_vs_evil("0 0 1 0 0 10", "0 0 1 0 0 0 10"),
        "Battle Result: Good triumphs over Evil"
    );
}

#[test]
fn returns_expected8() {
    assert_eq!(
        good_vs_evil("0 0 0 0 0 10", "1 0 0 0 0 0 10"),
        "Battle Result: Evil eradicates all trace of Good"
    );
}

#[test]
fn returns_expected9() {
    assert_eq!(
        good_vs_evil("0 0 0 1 0 10", "1 0 0 0 1 0 10"),
        "Battle Result: Evil eradicates all trace of Good"
    );
}

#[test]
fn returns_expected10() {
    assert_eq!(
        good_vs_evil("0 2 0 0 0 10", "0 0 0 1 1 0 10"),
        "Battle Result: Evil eradicates all trace of Good"
    );
}




