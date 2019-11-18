use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn is_valid(&self) -> bool {
        let mut numbers = HashSet::<u32>::new();
        let n = self.data.len() as u32;
        let sqrt_n = (n as f64).sqrt();
        if sqrt_n.trunc() != sqrt_n {
            return false;
        }
        let sqrt_n = sqrt_n as u32;

        for row in self.data.iter() {
            for number in row.iter() {
                if numbers.contains(number) || number > &n || number == &0 {
                    return false;
                }
                numbers.insert(*number);
            }
            if numbers.len() != n as usize || row.len() != n as usize {
                return false;
            }
            numbers.clear();
        }

        for column_index in 0..n - 1 {
            for row_index in 0..n - 1 {
                let number = self.data[row_index as usize][column_index as usize];

                if numbers.contains(&number) || number > n || number == 0 {
                    return false;
                }
                numbers.insert(number);
            }
            numbers.clear();
        }

        for square_col_index in 0..sqrt_n - 1 {
            for square_row_index in 0..sqrt_n - 1 {
                for row_index in 0..sqrt_n - 1 {
                    for col_index in 0..sqrt_n - 1 {
                        let number = self.data[(row_index + square_row_index * sqrt_n) as usize]
                            [(col_index + square_col_index * sqrt_n) as usize];
                        if numbers.contains(&number) {
                            return false;
                        }
                        numbers.insert(number);
                    }
                }
                numbers.clear()
            }
        }
        true
    }
}

#[test]
fn good_sudoku1() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };
    assert!(good_sudoku_1.is_valid());
}


#[test]
fn good_sudoku2() {

    let good_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };
    assert!(good_sudoku_2.is_valid());
}



#[test]
fn bad_sudoku1() {
    let bad_sudoku_1 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ],
    };

    assert!(!bad_sudoku_1.is_valid());
}

#[test]
fn bad_sudoku2() {

    let bad_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1],
        ],
    };
    assert!(!bad_sudoku_2.is_valid());
}

