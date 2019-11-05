use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

fn solution(n: f64) -> f64 {
    match n.fract().partial_cmp(&0.5) {
        Some(Ordering::Less) =>  match n.fract().partial_cmp(&0.25) {
            Some(Ordering::Less) => n.trunc(),
            _ => n.trunc() + 0.5
        },
        Some(Ordering::Greater) => match n.fract().partial_cmp(&0.75) {
            Some(Ordering::Less) => n.trunc() + 0.5,
            _ => n.trunc() + 1.
        },
        _ => n.trunc() + 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn sample_test1() {
        assert_eq!(solution(4.2), 4.0);
    }

    #[test]
    fn sample_test2() {
        assert_eq!(solution(4.4), 4.5);
    }

    #[test]
    fn sample_test3() {
        assert_eq!(solution(4.6), 4.5);
    }

    #[test]
    fn sample_test4() {
        assert_eq!(solution(4.75), 5.0);
    }

    #[test]
    fn sample_test5() {
         assert_eq!(solution(4.5), 4.5);
    }

    #[test]
    fn sample_test6() {
        assert_eq!(solution(4.), 4.);
    }

    #[test]
    fn sample_test7() {
        assert_eq!(solution(0.), 0.);
    }

    #[test]
    fn sample_test8() {
        assert_eq!(solution(0.1), 0.);
    }

    #[test]
    fn sample_test9() {
        assert_eq!(solution(0.9), 1.);
    }

    #[test]
    fn sample_test10() {
        assert_eq!(solution(0.01), 0.);
    }

}