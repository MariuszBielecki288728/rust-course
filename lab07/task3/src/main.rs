fn main() {
    println!("Hello, world!");
}

fn fcn(n: i32) -> i64 {
    1 << n
}

fn testequal(n: i32, exp: i64) -> () {
    assert_eq!(exp, fcn(n))
}
#[test]
fn basics() {
    testequal(17, 131072);
}

#[test]
fn basics0() {
    testequal(21, 2097152);
}

#[test]
fn basics1() {
    testequal(1, 2);
}

#[test]
fn basics2() {
    testequal(0, 1);
}

#[test]
fn basics3() {
    testequal(2, 4);
}

#[test]
fn basics4() {
    testequal(3, 8);
}

#[test]
fn basics5() {
    testequal(4, 16);
}

#[test]
fn basics6() {
    testequal(5, 32);
}

#[test]
fn basics7() {
    testequal(6, 64);
}

#[test]
fn basics8() {
    testequal(7, 128);
}
