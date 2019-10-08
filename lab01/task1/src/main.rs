
fn string_to_number(s: &str) -> i32 {
  s.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::string_to_number;
    
    #[test]
    fn returns_expected_above_0() {
      assert_eq!(string_to_number("1234"), 1234);
    }
    #[test]
    fn returns_expected_prefix_0() {
      assert_eq!(string_to_number("001405"), 1405);
    }
    #[test]
    fn returns_expected_below_0() {
      assert_eq!(string_to_number("-7"), -7);
    }
}