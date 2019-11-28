use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    chessboard_cell_color("A1", "A2");
}

fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    let cord_1 = str_to_pos(cell1);
    let cord_2 = str_to_pos(cell2);
    (cord_1.0 + cord_2.0) % 2 == (cord_1.1 + cord_2.1) % 2
}

fn str_to_pos(cell: &str) -> (i32, i32) {
    let letter_to_index: HashMap<char, i32> = [
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('D', 4),
        ('E', 5),
        ('F', 6),
        ('G', 7),
        ('H', 8),
    ]
    .iter()
    .cloned()
    .collect();
    (
        letter_to_index[&cell.chars().nth(0).unwrap()] as i32,
        cell.chars().nth(1).unwrap().to_digit(10).unwrap() as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(chessboard_cell_color("A1", "C3"), true);
    }

    #[test]
    fn basic_tests2() {
        assert_eq!(chessboard_cell_color("A1", "H3"), false);
    }

    #[test]
    fn basic_tests3() {
        assert_eq!(chessboard_cell_color("A1", "A2"), false);
    }

    #[test]
    fn basic_tests4() {
        assert_eq!(chessboard_cell_color("A1", "C1"), true);
    }

    #[test]
    fn basic_test5() {
        assert_eq!(chessboard_cell_color("A1", "A2"), false);
    }

    #[test]
    fn basic_test6() {
        assert_eq!(chessboard_cell_color("E1", "C3"), true);
    }

    #[test]
    fn basic_test7() {
        assert_eq!(chessboard_cell_color("A1", "A1"), true);
    }

    #[test]
    fn basic_test8() {
        assert_eq!(chessboard_cell_color("C2", "A2"), true);
    }

    #[test]
    fn basic_test10() {
        assert_eq!(chessboard_cell_color("B2", "A2"), false);
    }
}
