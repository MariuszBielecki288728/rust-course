use std::cmp;

enum Direction {
    Left,
    Rigth,
    Up,
    Down,
}

#[derive(Debug)]
enum Command {
    ChangeDirToLeft(i32),
    ChangeDirToRight(i32),
    MoveForward(i32),
}

pub fn execute(code: &str) -> String {
    generate_grid(generate_path(parse(code)))
}

fn parse(code: &str) -> Vec<Command> {
    let mut digit_acc: Vec<char> = vec![];
    let mut res_vec: Vec<Command> = vec![];
    let mut chars = code.chars();
    let mut last_command = chars.next().unwrap_or('0');

    for ch in chars {
        if ch.is_digit(10) {
            digit_acc.push(ch);
        } else {
            match make_command(last_command, &digit_acc) {
                Some(comm) => {
                    res_vec.push(comm);
                }
                None => {}
            }
            last_command = ch;
            digit_acc.clear();
        }
    }
    match make_command(last_command, &digit_acc) {
        Some(comm) => {
            res_vec.push(comm);
        }
        None => {}
    }
    res_vec
}

fn make_command(ch: char, repeat_raw: &Vec<char>) -> Option<Command> {
    let mut repeat = 1;
    if !repeat_raw.is_empty() {
        repeat = repeat_raw
            .iter()
            .cloned()
            .collect::<String>()
            .parse()
            .unwrap()
    }
    match ch {
        'F' => Some(Command::MoveForward(repeat)),
        'L' => Some(Command::ChangeDirToLeft(repeat)),
        'R' => Some(Command::ChangeDirToRight(repeat)),
        _ => None,
    }
}

fn generate_path(commands: Vec<Command>) -> Vec<(i32, i32)> {
    let mut current_direction: Direction = Direction::Rigth;
    let mut path: Vec<(i32, i32)> = vec![(0, 0)];
    let mut current_pos: (i32, i32) = (0, 0);

    for command in commands.into_iter() {
        match command {
            Command::MoveForward(n) => {
                for _ in 0..n {
                    current_pos = move_forward(current_pos, &current_direction);
                    path.push(current_pos);
                }
            }
            Command::ChangeDirToLeft(n) => {
                for _ in 0..n {
                    current_direction = change_direction_left(current_direction);
                }
            }
            Command::ChangeDirToRight(n) => {
                for _ in 0..n {
                    current_direction = change_direction_right(current_direction);
                }
            }
        }
    }
    path
}

fn move_forward(actual_pos: (i32, i32), dir: &Direction) -> (i32, i32) {
    let move_vec: (i32, i32) = get_direction_vec(dir);
    (actual_pos.0 + move_vec.0, actual_pos.1 + move_vec.1)
}

fn get_direction_vec(dir: &Direction) -> (i32, i32) {
    match &dir {
        Direction::Left => (-1, 0),
        Direction::Rigth => (1, 0),
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
    }
}

fn change_direction_left(actual_dir: Direction) -> Direction {
    match actual_dir {
        Direction::Down => Direction::Rigth,
        Direction::Rigth => Direction::Up,
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
    }
}

fn change_direction_right(actual_dir: Direction) -> Direction {
    match actual_dir {
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Rigth,
        Direction::Rigth => Direction::Down,
    }
}

fn generate_grid(path: Vec<(i32, i32)>) -> String {
    //let (x, y): (Vec<i32>, Vec<i32>) = path.into_iter().unzip();
    let (x_min, x_max): (i32, i32) = path.iter().cloned().fold((0, 0), |(min, max), (x, _)| {
        (cmp::min(min, x), cmp::max(max, x))
    });
    let (y_min, y_max): (i32, i32) = path.iter().cloned().fold((0, 0), |(min, max), (_, y)| {
        (cmp::min(min, y), cmp::max(max, y))
    });

    let grid_width = (x_min.abs() + x_max + 1) as usize;
    let grid_height = (y_min.abs() + y_max + 1) as usize;

    let mut grid = vec![vec![' '; grid_width]; grid_height];

    for (x, y) in path.into_iter() {
        grid[(y + y_min.abs()) as usize][(x + x_min.abs()) as usize] = '*';
    }

    grid.into_iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\r\n")
}

#[cfg(test)]
macro_rules! expect_equal {
    ($actual:expr, $expected:expr $(,)*) => {{
        let actual = $actual;
        let expected = $expected;
        assert_eq!(
            actual, expected,
            "\ngot:\n{}\n\nexpected:\n{}\n",
            actual, expected
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn example1() {
        expect_equal!(execute(""), "*");
    }

    #[test]
    fn example2() {
        expect_equal!(execute("FFFFF"), "******");
    }

    #[test]
    fn example3() {
        expect_equal!(
            execute("FFFFFLFFFFFLFFFFFLFFFFFL"),
            "******\r\n*    *\r\n*    *\r\n*    *\r\n*    *\r\n******",
        );
    }

    #[test]
    fn example4() {
        expect_equal!(
            execute("LFFFFFRFFFRFFFRFFFFFFF"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
    }

    #[test]
    fn example5() {
        expect_equal!(
            execute("LF5RF3RF3RF7"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
    }

    #[test]
    fn example6() {
        expect_equal!(execute("R"), "*",);
    }

    #[test]
    fn example8() {
        expect_equal!(execute("RLRLLLL4"), "*",);
    }

    #[test]
    fn exampl9() {
        expect_equal!(execute("F"), "**",);
    }

    #[test]
    fn example10() {
        expect_equal!(execute("RF"), "*\r\n*",);
    }
}
