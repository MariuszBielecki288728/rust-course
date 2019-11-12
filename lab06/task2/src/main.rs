fn main() {
    println!("Hello, world!");
}

fn dig_pow(n: i64, p: i32) -> i64 {
    let result = n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .zip(p..p + n.to_string().chars().count() as i32)
        .fold(0_i64, |n, (x, y)| n + x.pow(y as u32) as i64) as f64
        / n as f64;

    if result.trunc() == result {
        result.trunc() as i64
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn test1() {
        dotest(89, 1, 1);
    }

    #[test]
    fn test2() {
        dotest(92, 1, -1);
    }

    #[test]
    fn test3() {
        dotest(46288, 3, 51);
    }

    #[test]
    fn test4() {
        dotest(1, 3, 1);
    }

    #[test]
    fn test6() {
        dotest(12314, 1, -1);
    }

    #[test]
    fn test7() {
        dotest(11111, 3, -1);
    }


    #[test]
    fn test8() {
        dotest(1415123, 3, -1);
    }


    #[test]
    fn test10() {
        dotest(10, 10, -1);
    }
}
