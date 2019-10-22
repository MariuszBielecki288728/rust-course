fn main() {
    next_bigger_number(1000);
}

fn next_bigger_number(n: i64) -> i64 {
    print!("{}", n.to_string().to_iter().rev());
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
    }
}
