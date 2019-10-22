fn main() {
    println!("Hello, world!");
}

fn summy(strng: &str) -> i32 {
    strng.split_whitespace().map(|s| s.parse::<i32>().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test1() {
        assert_eq!(summy("1 2 3"), 6);
    }
    #[test]
    fn sample_test2() {
        assert_eq!(summy("1 2 3 4"), 10);
    }

    #[test]
    fn sample_test3() {
        assert_eq!(summy("1 2 3 4 5"), 15);
    }

    #[test]
    fn sample_test4() {
        assert_eq!(summy("10 10"), 20);
    }

    #[test]
    fn sample_test5() {
        assert_eq!(summy("0"), 0);
    }

    #[test]
    fn sample_test6() {
        assert_eq!(summy("1"), 1);
    }

    #[test]
    fn sample_test7() {
        assert_eq!(summy("100 0"), 100);
    }

    #[test]
    fn sample_test8() {
        assert_eq!(summy("0 0 0 0"), 0);
    }

    #[test]
    fn sample_test9() {
        assert_eq!(summy("00"), 0);
    }

    #[test]
    fn sample_test10() {
        assert_eq!(summy("-1"), -1);
    }
}