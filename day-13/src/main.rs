use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    multi::{many1, separated_list1},
    IResult,
};
use simple_matrix::Matrix;

fn parse_mat(input: &str) -> IResult<&str, Matrix<char>> {
    let (input, lines) = separated_list1(tag("\n"), many1(one_of(&['.', '#'][..])))(input)?;

    Ok((
        input,
        Matrix::from_iter(lines.len(), lines[0].len(), lines.iter().flatten().copied()),
    ))
}

fn parse_mats(input: &str) -> IResult<&str, Vec<Matrix<char>>> {
    let (input, mats) = separated_list1(tag("\n\n"), parse_mat)(input)?;

    Ok((input, mats))
}

fn task_1(input: &str) -> u32 {
    let matrices = parse_mats(input).unwrap().1;

    matrices
        .iter()
        .map(|mat| {
            let mut res = 0;

            for (l, r) in (0..mat.cols()).tuple_windows() {
                let mut left = mat.get_col(l).unwrap().collect_vec();
                let mut right = mat.get_col(r).unwrap().collect_vec();

                let mut i = 1;
                while left == right {
                    if i > l {
                        res = (l + 1) as u32;
                        break;
                    }
                    if let Some(l) = mat.get_col(l - i) {
                        left = l.collect_vec();
                    } else {
                        if l - i == 0 {
                            res = (l + 1) as u32;
                        }
                        break;
                    }
                    if let Some(r) = mat.get_col(r + i) {
                        right = r.collect_vec();
                    } else {
                        if r + i == mat.cols() {
                            res = (l + 1) as u32;
                        }
                        break;
                    }
                    i += 1;
                }
            }

            for (t, b) in (0..mat.rows()).tuple_windows() {
                let mut top = mat.get_row(t).unwrap().collect_vec();
                let mut bottom = mat.get_row(b).unwrap().collect_vec();

                let mut i = 1;
                while top == bottom {
                    if i > t {
                        res = ((t + 1) * 100) as u32;
                        break;
                    }
                    if let Some(t) = mat.get_row(t - i) {
                        top = t.collect_vec();
                    } else {
                        if t - i == 0 {
                            res = ((t + 1) * 100) as u32;
                        }
                        break;
                    }
                    if let Some(b) = mat.get_row(b + i) {
                        bottom = b.collect_vec();
                    } else {
                        if b + i == mat.rows() {
                            res = ((t + 1) * 100) as u32;
                        }
                        break;
                    }
                    i += 1;
                }
            }

            res
        })
        .sum()
}

fn task_2(input: &str) -> u32 {
    let matrices = parse_mats(input).unwrap().1;

    matrices
        .iter()
        .map(|mat| {
            let mut res = 0;

            for (l, r) in (0..mat.cols()).tuple_windows() {
                let mut left = mat.get_col(l).unwrap().collect_vec();
                let mut right = mat.get_col(r).unwrap().collect_vec();

                let mut changed_char = false;

                let mut i = 1;
                while left == right || (left
                        .iter()
                        .zip(right.clone())
                        .filter(|(a, b)| *a != b)
                        .count()
                        == 1
                        && changed_char == false) {
                    if left != right {
                        changed_char = true;
                    }

                    if i > l {
                        if changed_char {
                            res = (l + 1) as u32;
                        }
                        break;
                    }
                    if let Some(l) = mat.get_col(l - i) {
                        left = l.collect_vec();
                    } else {
                        if l - i == 0 && changed_char {
                            res = (l + 1) as u32;
                        }
                        break;
                    }
                    if let Some(r) = mat.get_col(r + i) {
                        right = r.collect_vec();
                    } else {
                        if r + i == mat.cols() && changed_char  {
                            res = (l + 1) as u32;
                        }
                        break;
                    }
                    i += 1;
                }
            }

            for (t, b) in (0..mat.rows()).tuple_windows() {
                let mut top = mat.get_row(t).unwrap().collect_vec();
                let mut bottom = mat.get_row(b).unwrap().collect_vec();

                let mut changed_char = false;

                let mut i = 1;
                while top == bottom
                    || (top
                        .iter()
                        .zip(bottom.clone())
                        .filter(|(a, b)| *a != b)
                        .count()
                        == 1
                        && changed_char == false)
                {
                    if top != bottom {
                        changed_char = true;
                    }

                    if i > t {
                        if changed_char {
                            res = ((t + 1) * 100) as u32;
                        }
                        break;
                    }
                    if let Some(t) = mat.get_row(t - i) {
                        top = t.collect_vec();
                    } else {
                        if t - i == 0 && changed_char {
                            res = ((t + 1) * 100) as u32;
                        }
                        break;
                    }
                    if let Some(b) = mat.get_row(b + i) {
                        bottom = b.collect_vec();
                    } else {
                        if b + i == mat.rows() && changed_char {
                            res = ((t + 1) * 100) as u32;
                        }
                        break;
                    }
                    i += 1;
                }
            }

            // if res == 0 {
            //     panic!("res should be > 0");
            // }
            dbg!(res);

            res
        })
        .sum()
}

fn main() {
    let input = include_str!("./data.txt");

    println!("task 1: {}", task_1(input));
    println!("task 2: {}", task_2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_1_works() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let result = task_1(input);
        assert_eq!(result, 405); 
    }

    #[test]
    fn task_2_works() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let result = task_2(input);
        assert_eq!(result, 400);
    }
}
