fn main() {
    println!("{:?}", encode("masterpiece".to_string(), 1));
}

fn encode(msg: String, n: i32) -> Vec<i32> {
    msg.chars()
        .map(|x| (x as i32) - ('a' as i32) + 1)
        .zip(
            n.to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .cycle(),
        )
        .map(|(x, y)| x + y as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
    }

    #[test]
    fn test2() {
        assert_eq!(
            encode("masterpiece".to_string(), 1939),
            vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            encode("a".to_string(), 1),
            vec![2]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            encode("a".to_string(), 0),
            vec![1]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            encode("b".to_string(), 1),
            vec![3]
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            encode("a".to_string(), 11),
            vec![2]
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            encode("aa".to_string(), 1),
            vec![2, 2]
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            encode("abc".to_string(), 0),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test10() {
        assert_eq!(
            encode("abc".to_string(), 321),
            vec![4, 4, 4]
        );
    }
}
