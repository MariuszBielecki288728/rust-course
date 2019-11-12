fn main() {
    println!("Hello, world!");
    println!("{:?}", print(5).unwrap());
}

fn print(n: i32) -> Option<String> {
    if n % 2 == 0 || n < 0 {
        None
    } else {
        let mut s1: Vec<String> = vec![];
        for i in 1..(n + 1) / 2 {
            let mut s = std::iter::repeat(" ").take(i as usize).collect::<String>();
            s.push_str(
                &std::iter::repeat("*")
                    .take((n - 2 * i) as usize)
                    .collect::<String>(),
            );

            s1.push(s)
        }

        let s2 = std::iter::repeat("*").take(n as usize).collect::<String>();

        let mut s3 = s1.iter().cloned().rev().collect::<Vec<_>>();

        s3.push(s2);
        s3.extend(s1);
        Some(format!("{}", s3.join("\n") + &"\n".to_string()))
    }
}

#[test]
fn basic_test1() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
}

#[test]
fn basic_test2() {
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
}

#[test]
fn basic_test3() {
    assert_eq!(print(-3), None);
}

#[test]
fn basic_test4() {
    assert_eq!(print(2), None);
}

#[test]
fn basic_test5() {
    assert_eq!(print(0), None);
}

#[test]
fn basic_test6() {
    assert_eq!(print(1), Some("*\n".to_string()));
}

#[test]
fn basic_test7() {
    assert_eq!(print(-23), None);
}

#[test]
fn basic_test8() {
    assert_eq!(print(22), None);
}

#[test]
fn basic_test9() {
    assert_eq!(print(10), None);
}

#[test]
fn basic_test10() {
    assert_eq!(print(-1234), None);
}



