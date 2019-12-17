fn main() {
    println!("Hello, world!");
}
fn first_letter_to_uppper_case(s1: &str) -> String {
    let mut ch_iter = s1.chars();
    match ch_iter.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + ch_iter.as_str(),
    }
}

fn camel_case(str: &str) -> String {
    str.trim()
        .split_whitespace()
        .map(first_letter_to_uppper_case)
        .collect::<Vec<String>>()
        .join("")
}

// Rust tests
#[test]
fn sample_test1() {
    assert_eq!(camel_case("test case"), "TestCase");
}
#[test]
fn sample_test2() {
    assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
}
#[test]
fn sample_test3() {
    assert_eq!(camel_case("say hello "), "SayHello");
}
#[test]
fn sample_test4() {
    assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
}
#[test]
fn sample_test5() {
    assert_eq!(camel_case(""), "");
}
#[test]
fn sample_test6() {
    assert_eq!(camel_case("ok boomer"), "OkBoomer");
}
#[test]
fn sample_test7() {
    assert_eq!(camel_case("a"), "A");
}
#[test]
fn sample_test8() {
    assert_eq!(camel_case("a b"), "AB");
}
#[test]
fn sample_test9() {
    assert_eq!(camel_case("ab c"), "AbC");
}
#[test]
fn sample_test10() {
    assert_eq!(camel_case("eh"), "Eh");
}
