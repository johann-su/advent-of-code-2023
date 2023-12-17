use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn of(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Galaxy,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map(Vec<Vec<Field>>);

impl Map {
    fn get(&self, position: Pos) -> Option<Field> {
        if position.x < 0 || position.y < 0 {
            None
        } else {
            if let Some(row) = self.0.get(position.y as usize) {
                if let Some(item) = row.get(position.x as usize) {
                    return Some(*item);
                }
            }
            None
        }
    }

    fn get_row(&self, i: usize) -> Option<Vec<Field>> {
        if let Some(row) = self.0.get(i) {
            Some(row.to_vec())
        } else {
            None
        }
    }

    fn cols_iter(&self) -> impl Iterator<Item = Vec<Field>> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || {
            let res = self.get_col(i);
            i += 1;
            res
        })
    }

    fn get_col(&self, i: usize) -> Option<Vec<Field>> {
        let mut col: Vec<Field> = vec![];
        for row in self.0.iter() {
            if let Some(v) = row.get(i) {
                col.push(*v);
            } else {
                return None;
            }
        }
        Some(col)
    }

    fn iter_pos(&self) -> impl Iterator<Item = (Field, Pos)> + '_ {
        let mut x = 0;
        let mut y = 0;
        let line_len = self.get_row(0).unwrap().len() - 1;
        std::iter::from_fn(move || {
            let pos = Pos::of(x, y);
            if let Some(field) = self.get(pos) {
                if x == line_len as i32 {
                    x = 0;
                    y += 1;
                } else {
                    x += 1;
                }
                Some((field, pos))
            } else {
                None
            }
        })
    }

    fn galaxies_pos(&self) -> Vec<Pos> {
        let mut positions: Vec<Pos> = vec![];
        for (item, Pos { x, y }) in self.iter_pos() {
            if item == Field::Galaxy {
                positions.push(Pos::of(x as i32, y as i32))
            }
        }

        positions
    }

    fn expand(&mut self) {
        let mut i = 0;
        while let Some(row) = self.get_row(i) {
            if row.iter().all_equal() {
                i += 1;
                self.0.insert(i, vec![Field::Empty; row.len()]);
            }
            i += 1;
        }

        i = 0;
        while let Some(col) = self.get_col(i) {
            if col.iter().all_equal() {
                i += 1;
                for row in self.0.iter_mut() {
                    row.insert(i, Field::Empty);
                }
            }
            i += 1;
        }
    }

    fn expansions(&self) -> (Vec<i32>, Vec<i32>) {
        let mut expansions_col: Vec<i32> = vec![];
        let mut expansions_row: Vec<i32> = vec![];

        for (i, row) in self.0.iter().enumerate() {
            if row.iter().all_equal() {
                expansions_row.push(i as i32);
            }
        }
        for (i, col) in self.cols_iter().enumerate() {
            if col.iter().all_equal() {
                expansions_col.push(i as i32);
            }
        }

        (expansions_col, expansions_row)
    }
}

fn parse_input(input: &str) -> IResult<&str, Map> {
    let (input, galaxies) = separated_list1(
        tag("\n"),
        many1(|line| {
            map_res(anychar, |char| match char {
                '.' => Ok(Field::Empty),
                '#' => Ok(Field::Galaxy),
                _ => Err("not a valid character"),
            })(line)
        }),
    )(input)?;

    Ok((input, Map(galaxies)))
}

fn task_1(input: &str) -> u32 {
    let mut map = parse_input(input).unwrap().1;
    map.expand();
    let galaxies = map.galaxies_pos();

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
        .sum()
}

fn task_2(input: &str, expansion_factor: u32) -> u64 {
    let map = parse_input(input).unwrap().1;
    let galaxies = map.galaxies_pos();
    let expansions = map.expansions();

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let expansions_a = (
                expansions
                    .0
                    .iter()
                    .fold(0, |acc, n| if n < &&a.x { acc + 1 } else { acc }),
                expansions
                    .1
                    .iter()
                    .fold(0, |acc, n| if n < &&a.y { acc + 1 } else { acc }),
            );
            let shifted_a = (
                a.x + (expansions_a.0 * (expansion_factor-1)) as i32,
                a.y + (expansions_a.1 * (expansion_factor-1)) as i32,
            );

            let expansions_b = (
                expansions
                    .0
                    .iter()
                    .fold(0, |acc, n| if n < &&b.x { acc + 1 } else { acc }),
                expansions
                    .1
                    .iter()
                    .fold(0, |acc, n| if n < &&b.y { acc + 1 } else { acc }),
            );
            let shifted_b = (
                b.x + (expansions_b.0 * (expansion_factor-1)) as i32,
                b.y + (expansions_b.1 * (expansion_factor-1)) as i32,
            );

            (shifted_a.0.abs_diff(shifted_b.0) + shifted_a.1.abs_diff(shifted_b.1)) as u64
        })
        .sum()
}

fn main() {
    let input = include_str!["./data.txt"];

    println!("task 1: {}", task_1(input));
    println!("task 2: {}", task_2(input, 1_000_000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_1_works() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = task_1(input);
        assert_eq!(result, 374);
    }

    #[test]
    fn task_2_works() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = task_2(input, 10);
        assert_eq!(result, 1030);

        let result = task_2(input, 100);
        assert_eq!(result, 8410);
    }
}
