fn main() {
    println!("Hello, world!");
}

fn last_digit(lst: &[u64]) -> u64 {
    lst.iter().rev().fold(1, |acc, &val| {
        (if val < 20 { val } else { val % 20 + 20 })
            .pow((if acc < 4 { acc } else { (acc % 4 + 4) }) as u32)
    }) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test = (vec![], 1);
        assert_eq!(last_digit(&test.0), test.1);
    }

    #[test]
    fn test2() {
        let test = (vec![0, 0], 1);
        assert_eq!(last_digit(&test.0), test.1);
    }

    #[test]
    fn test3() {
        let test = (vec![0, 0, 0], 0);
        assert_eq!(last_digit(&test.0), test.1);
    }

    #[test]
    fn test4() {
        let test = (vec![1, 2], 1);
        assert_eq!(last_digit(&test.0), test.1);
    }

    #[test]
    fn test5() {
        let test = (vec![3, 4, 5], 1);
        assert_eq!(last_digit(&test.0), test.1);
    }
    #[test]
    fn test6() {
        let test = (vec![4, 3, 6], 4);
        assert_eq!(last_digit(&test.0), test.1);
    }

    #[test]
    fn test7() {
        let test = (vec![7, 6, 21], 1);
        assert_eq!(last_digit(&test.0), test.1);
    }

    #[test]
    fn test8() {
        let test = (vec![7, 6, 21], 1);
        assert_eq!(last_digit(&test.0), test.1);
    }

    #[test]
    fn test9() {
        let test = (vec![12, 30, 21], 6);
        assert_eq!(last_digit(&test.0), test.1);
    }

    #[test]
    fn test10() {
        let test = (vec![123232, 694022, 140249], 6);
        assert_eq!(last_digit(&test.0), test.1);
    }
}
