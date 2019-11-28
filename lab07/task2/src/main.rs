fn main() {
    println!("Hello, world!");
}

use std::collections::BTreeMap;

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    let mut map = BTreeMap::<char, i32>::new();
    for ch in input.chars().map(|x| x.to_lowercase().nth(0).unwrap()) {
        if !ch.is_whitespace() {
            map.insert(ch, map.get(&ch).unwrap_or(&0) + 1);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::letter_frequency;
    use std::collections::BTreeMap;

    #[test]
    fn simpleword() {
        let answer: BTreeMap<char, i32> = [('a', 2), ('c', 1), ('l', 1), ('t', 1), ('u', 1)]
            .iter()
            .cloned()
            .collect();

        assert_eq!(letter_frequency("actual"), answer);
    }

    #[test]
    fn sequence() {
        let answer: BTreeMap<char, i32> = [
            ('a', 3),
            ('b', 2),
            ('f', 1),
            ('p', 1),
            ('s', 1),
            ('t', 2),
            ('u', 1),
            ('x', 5),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(letter_frequency("AaabBF UttsP xxxxx"), answer);
    }

    #[test]
    fn sequence2() {
        let answer: BTreeMap<char, i32> = [('a', 2)].iter().cloned().collect();

        assert_eq!(letter_frequency("Aa"), answer);
    }

    #[test]
    fn sequence3() {
        let answer: BTreeMap<char, i32> = [('a', 2)].iter().cloned().collect();

        assert_eq!(letter_frequency("    A    a  "), answer);
    }

    #[test]
    fn sequence4() {
        let answer: BTreeMap<char, i32> = [].iter().cloned().collect();

        assert_eq!(letter_frequency(""), answer);
    }

    #[test]
    fn sequence5() {
        let answer: BTreeMap<char, i32> = [('ą', 1)].iter().cloned().collect();

        assert_eq!(letter_frequency("Ą"), answer);
    }

    #[test]
    fn sequence6() {
        let answer: BTreeMap<char, i32> = [('a', 1), ('b', 1), ('c', 1)].iter().cloned().collect();

        assert_eq!(letter_frequency("a b c"), answer);
    }

    #[test]
    fn sequence7() {
        let answer: BTreeMap<char, i32> = [('a', 4)].iter().cloned().collect();

        assert_eq!(letter_frequency("a a  a  a"), answer);
    }

    #[test]
    fn sequence8() {
        let answer: BTreeMap<char, i32> = [].iter().cloned().collect();

        assert_eq!(letter_frequency("                "), answer);
    }

    #[test]
    fn sequence9() {
        let answer: BTreeMap<char, i32> = [('f', 1), ('o', 2)].iter().cloned().collect();

        assert_eq!(letter_frequency("foo"), answer);
    }
}
