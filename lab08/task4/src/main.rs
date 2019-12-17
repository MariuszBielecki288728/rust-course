use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

// Preloaded:
//
struct MorseDecoder {
    morse_code: HashMap<String, String>,
}
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

impl MorseDecoder {
    fn new() -> MorseDecoder {
        MorseDecoder {
            morse_code: [
                ("....-", "4"),
                ("--..--", ","),
                (".--", "W"),
                (".-.-.-", "."),
                ("..---", "2"),
                (".", "E"),
                ("--..", "Z"),
                (".----", "1"),
                (".-..", "L"),
                (".--.", "P"),
                (".-.", "R"),
                ("...", "S"),
                ("-.--", "Y"),
                ("...--", "3"),
                (".....", "5"),
                ("--.", "G"),
                ("-.--.", "("),
                ("-....", "6"),
                (".-.-.", "+"),
                ("...-..-", "$"),
                (".--.-.", "@"),
                ("...---...", "SOS"),
                ("..--.-", "_"),
                ("-.", "N"),
                ("-..-", "X"),
                ("-----", "0"),
                ("....", "H"),
                ("-...", "B"),
                (".---", "J"),
                ("---...", ","),
                ("-", "T"),
                ("---..", "8"),
                ("-..-.", "/"),
                ("--.-", "Q"),
                ("...-", "V"),
                ("----.", "9"),
                ("--", "M"),
                ("-.-.-.", ";"),
                ("-.-.--", "!"),
                ("..-.", "F"),
                ("..--..", "?"),
                ("-...-", "="),
                ("..-", "U"),
                (".----.", "'"),
                ("---", "O"),
                ("-.--.-", ")"),
                ("..", "I"),
                ("-....-", "-"),
                (".-..-.", "\""),
                (".-", "A"),
                ("-.-.", "C"),
                ("-..", "D"),
                (".-...", "&"),
                ("--...", "7"),
                ("-.-", "K"),
            ]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        }
    }
    fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .trim()
            .split("   ")
            .map(|word| {
                word.split_whitespace()
                    .map(|code| &self.morse_code[code] as &str)
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
            "HEY JUDE"
        );
    }

    #[test]
    fn test2() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(".- -- . -... -.- .-"), "AMEBKA");
    }

    #[test]
    fn test3() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(".-   -...   -.-."), "A B C");
    }
    #[test]
    fn test4() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse("   .-   "), "A");
    }

    #[test]
    fn test5() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(""), "");
    }

    #[test]
    fn test6() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(".-   -..."), "A B");
    }

    #[test]
    fn test7() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(".- -...   -.-."), "AB C");
    }

    #[test]
    fn test8() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(".-   -... -.-."), "A BC");
    }

    #[test]
    fn test9() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse("-..- --- -..- ---"), "XOXO");
    }
    #[test]
    fn test10() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse("...---..."), "SOS");
    }
}
