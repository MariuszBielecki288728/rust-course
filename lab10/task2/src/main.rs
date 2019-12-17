use std::collections::HashMap;

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        return "".to_string();
    }
    let mut cats: HashMap<char, i32> = HashMap::new();
    for (category, value) in list_art.iter().map(|s| {
        (
            s.split_whitespace().nth(0).unwrap().chars().next().unwrap(),
            s.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap(),
        )
    }) {
        *cats.entry(category).or_insert(0) += value;
    }

    list_cat
        .iter()
        .map(|s| {
            format!(
                "({} : {})",
                s,
                *cats.entry(s.chars().next().unwrap()).or_insert(0)
            )
        })
        .collect::<Vec<_>>()
        .join(" - ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_test1() {
        let b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");
    }
    #[test]
    fn basic_test2() {
        let b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");
    }

    #[test]
    fn basic_test3() {
        let b = vec!["BTSQ 890", "DRTY 600"];
        let c = vec!["A", "B"];
        dotest(b, c, "(A : 0) - (B : 890)");
    }

    #[test]
    fn basic_test4() {
        let b = vec!["ABAR 200"];
        let c = vec!["A"];
        dotest(b, c, "(A : 200)");
    }

    #[test]
    fn basic_test5() {
        let b = vec!["AB 123"];
        let c = vec!["A", "B"];
        dotest(b, c, "(A : 123) - (B : 0)");
    }

    #[test]
    fn basic_test6() {
        let b = vec!["ABAR 999", "AAAA 1"];
        let c = vec!["A"];
        dotest(b, c, "(A : 1000)");
    }

    #[test]
    fn basic_test7() {
        let b = vec![];
        let c = vec!["A", "B"];
        dotest(b, c, "");
    }

    #[test]
    fn basic_test8() {
        let b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let c = vec![];
        dotest(b, c, "");
    }

    #[test]
    fn basic_test9() {
        let b = vec!["BKWR 250", "BTSQ 890", "DRTY 600"];
        let c = vec!["E"];
        dotest(b, c, "(E : 0)");
    }

    #[test]
    fn basic_test10() {
        let b = vec!["A 0"];
        let c = vec!["A", "B"];
        dotest(b, c, "(A : 0) - (B : 0)");
    }
}
