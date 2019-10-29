fn main() {
    println!("Hello, world!");
}

#[derive(Copy, Clone, PartialEq)]
enum TokenType {
    FCommand,
    LCommand,
    RCommand,
    Digit,
    Other
}

const COLORS: &'static [&'static str] = &[
    "pink",
    "red",
    "green",
    "orange"
];

pub fn highlight(code: &str) -> String {
  let mut res_vector: Vec<String> = vec!();
  let mut accumulator: String = "".to_string();
  let mut current_processed_type: TokenType = TokenType::Other;

  for ch in code.chars() {
    let type_ = find_token_type(&ch);
    if type_ != current_processed_type {
        res_vector.push(generate_highlighting(&current_processed_type, &accumulator));
        current_processed_type = type_;
        accumulator = ch.clone().to_string();
    } else {
        accumulator.push(ch)
    }
  }
  res_vector.push(generate_highlighting(&current_processed_type, &accumulator));
  res_vector.join("")
}

fn find_token_type(ch: &char) -> TokenType {
    match ch {
        'F' => TokenType::FCommand,
        'L' => TokenType::LCommand,
        'R' => TokenType::RCommand,
        ch if ch.is_digit(10) => TokenType::Digit,
        _   => TokenType::Other
    }
}

fn generate_highlighting(type_: &TokenType, string: &String) -> String {
    match type_ {
        TokenType::Other => string.clone(),
        typ => format!("<span style=\"color: {}\">{}</span>", COLORS[*typ as usize], string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    macro_rules! assert_highlight {
        ($code:expr , $expected:expr $(,)*) => {{
            let actual = highlight($code);
            let expected = $expected;
            println!("Code without syntax highlighting: {}", $code);
            println!("Your code with syntax highlighting: {}", actual);
            println!("Expected syntax highlighting: {}", expected);
            assert_eq!(actual, expected);
        }};
    }

    #[test]
    fn example1() {
        assert_highlight!(
            "F",
            r#"<span style="color: pink">F</span>"#,
        );
    }

    #[test]
    fn example2() {
        assert_highlight!(
            "{}",
            r#"{}"#,
        );
    }

    #[test]
    fn example3() {
        assert_highlight!(
            "L",
            r#"<span style="color: red">L</span>"#
        );
    }

    #[test]
    fn example4() {
        assert_highlight!(
            "RRR",
            r#"<span style="color: green">RRR</span>"#,
        );
    }

    #[test]
    fn example5() {
        assert_highlight!(
            "123",
            r#"<span style="color: orange">123</span>"#,
        );
    }

    #[test]
    fn example6() {
        assert_highlight!(
            "",
            r#""#,
        );
    }

    #[test]
    fn example7() {
        assert_highlight!(
            "  ",
            r#"  "#,
        );
    }

    #[test]
    fn example8() {
        assert_highlight!(
            "{FR1L}",
            r#"{<span style="color: pink">F</span><span style="color: green">R</span><span style="color: orange">1</span><span style="color: red">L</span>}"#,
        );
    }

    #[test]
    fn example9() {
        assert_highlight!(
            "F3RF5LF7",
            r#"<span style="color: pink">F</span><span style="color: orange">3</span><span style="color: green">R</span><span style="color: pink">F</span><span style="color: orange">5</span><span style="color: red">L</span><span style="color: pink">F</span><span style="color: orange">7</span>"#,
        );
    }

    #[test]
    fn example10() {
        assert_highlight!(
            "FFFR345F2LL",
            r#"<span style="color: pink">FFF</span><span style="color: green">R</span><span style="color: orange">345</span><span style="color: pink">F</span><span style="color: orange">2</span><span style="color: red">LL</span>"#,
        );
    }
}