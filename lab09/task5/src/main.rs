fn main() {
    println!("Hello, world!");
}

mod solution {
    use itertools::Itertools;

    pub fn range_extraction(a: &[i32]) -> String {
        let mut res: Vec<String> = vec![];
        for (_, group) in &a
            .into_iter()
            .enumerate()
            .group_by(|(i, &x)| (*i as i32) - x)
        {
            let grp_vec = group.map(|(_, v)| v).collect::<Vec<&i32>>();
            if grp_vec.len() > 2 {
                res.push(format!(
                    "{}-{}",
                    grp_vec.first().unwrap(),
                    grp_vec.last().unwrap()
                ));
            } else {
                res.append(&mut grp_vec.iter().cloned().map(|&i| i.to_string()).collect());
            }
        }
        res.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
    }

    #[test]
    fn example6() {
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }

    #[test]
    fn example7() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
    }

    #[test]
    fn example8() {
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }

    #[test]
    fn example9() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
    }

    #[test]
    fn example10() {
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }
}
