fn main() {
    println!("Hello, world!");
}

fn game(n: u64) -> Vec<u64> {
    if n % 2 == 0 {
        return vec![n * n / 2];
    }

    return vec![n * n, 2];
}

fn testing(n: u64, exp: Vec<u64>) -> () {
    assert_eq!(game(n), exp)
}

#[test]
fn basics_game1() {
    testing(204, vec![20808]);
}

#[test]
fn basics_game2() {
    testing(807, vec![651249, 2]);
}

#[test]
fn basics_game3() {
    testing(5014, vec![12570098]);
}

#[test]
fn basics_game4() {
    testing(750001, vec![562501500001, 2]);
}

#[test]
fn basics_game5() {
    testing(420, vec![88200]);
}

#[test]
fn basics_game6() {
    testing(0, vec![0]);
}

#[test]
fn basics_game7() {
    testing(111, vec![12321, 2]);
}

#[test]
fn basics_game8() {
    testing(420, vec![88200]);
}

#[test]
fn basics_game9() {
    testing(420, vec![88200]);
}

#[test]
fn basics_game10() {
    testing(1, vec![1, 2]);
}