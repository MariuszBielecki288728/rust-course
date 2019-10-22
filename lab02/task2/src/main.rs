fn main() {
    println!("Hello, world!");
}

fn longest(a1: &str, a2: &str) -> String {
    let mut vec: Vec<char> = [a1, a2].join("").chars().collect();
    vec.sort();
    vec.dedup();
    vec.iter().collect::<String>()
}

#[cfg(test)]
    mod tests {
    use super::*;
   
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_test1() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");        
    }

    #[test]
    fn basic_test2() {
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
    }

    #[test]
    fn basic_test3() {
        testing("abc", "abc", "abc");
    }

    #[test]
    fn basic_test4() {
        testing("abc", "d", "abcd");
    }

    #[test]
    fn basic_test5() {
        testing("a", "bcd", "abcd");
    }

    #[test]
    fn basic_test6() {
        testing("a", "a", "a");
    }

    #[test]
    fn basic_test7() {
        testing("abcd", "cdef", "abcdef");
    }

    #[test]
    fn basic_test8() {
        testing("aa", "bb", "ab");
    }

    #[test]
    fn basic_test9() {
        testing("aaa", "aaa", "a");
    }

    #[test]
    fn basic_test10() {
        testing("bcd", "abc", "abcd");
    }
}
