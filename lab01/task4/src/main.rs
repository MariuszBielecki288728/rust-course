fn printer_error(s: &str) -> String {
    format!(
        "{}/{}",
        s.chars().filter(|ch| ch > &'m').count(),
        s.chars().count()
    )
}

#[test]
fn returns_correct_format() {
    use regex::Regex;
    let re = Regex::new(r"^\d+/\d+").unwrap();
    assert!(re.is_match(&printer_error("abc")));
}

#[test]
fn works_with_all_colors_correct() {
    assert_eq!(printer_error("abc"), "0/3");
}

#[test]
fn works_with_incorrect_colors_end() {
    assert_eq!(printer_error("abcx"), "1/4");
}

#[test]
fn works_with_incorrect_colors_start() {
    assert_eq!(printer_error("zabc"), "1/4");
}

#[test]
fn works_with_incorrect_colors() {
    assert_eq!(printer_error("ajbpfcx"), "2/7");
}
