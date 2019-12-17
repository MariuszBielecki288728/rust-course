fn main() {
    println!("Hello, world!");
}

fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..(end + 1))
        .filter(|x| !x.to_string().contains("5"))
        .count() as isize
}

#[cfg(test)]
mod tests {
    use super::dont_give_me_five;

    #[test]
    fn returns_expected1() {
        assert_eq!(dont_give_me_five(1, 9), 8);
    }

    #[test]
    fn returns_expected2() {
        assert_eq!(dont_give_me_five(4, 17), 12);
    }

    #[test]
    fn returns_expected3() {
        assert_eq!(dont_give_me_five(4, 100), 78);
    }

    #[test]
    fn returns_expected4() {
        assert_eq!(dont_give_me_five(0, 123), 103);
    }

    #[test]
    fn returns_expected5() {
        assert_eq!(dont_give_me_five(1, 1000), 729);
    }

    #[test]
    fn returns_expected6() {
        assert_eq!(dont_give_me_five(1, 2), 2);
    }

    #[test]
    fn returns_expected7() {
        assert_eq!(dont_give_me_five(0, 4), 5);
    }

    #[test]
    fn returns_expected8() {
        assert_eq!(dont_give_me_five(5, 5), 0);
    }

    #[test]
    fn returns_expected9() {
        assert_eq!(dont_give_me_five(4, 6), 2);
    }

    #[test]
    fn returns_expected10() {
        assert_eq!(dont_give_me_five(5, 15), 9);
    }
}
