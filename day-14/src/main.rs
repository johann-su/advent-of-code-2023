use std::collections::HashMap;

use itertools::Itertools;
use simple_matrix::Matrix;

fn print_mat(mat: &Matrix<char>) {
    for i in 0..mat.rows() {
        for j in 0..mat.cols() {
            print!(" {} ", mat.get(i, j).unwrap());
        }
        print!("\n");
    }
}

fn move_stones(mat: &mut Matrix<char>) {
    for i in 0..mat.cols() {
        for j in 1..mat.rows() {
            if *mat.get(j, i).unwrap() == 'O' {
                let mut j = j;
                let char = mat.get_mut(j, i).unwrap();
                *char = '.';

                while let Some('.') = mat.get(j - 1, i) {
                    if j == 1 {
                        j = 0;
                        break;
                    }
                    j -= 1;
                }

                let char = mat.get_mut(j, i).unwrap();
                *char = 'O';
            }
        }
    }
}

fn task_1(input: &str) -> u32 {
    let mut mat: Matrix<char> = Matrix::from_iter(
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
        input.chars().filter(|c| ['.', 'O', '#'].contains(c)),
    );

    move_stones(&mut mat);

    let mut res = 0;
    for i in (0..mat.rows()).rev() {
        let row = mat.get_row(mat.rows() - 1 - i).unwrap();
        res += (i + 1) * row.filter(|char| **char == 'O').count()
    }

    res as u32
}

fn rotate_mat(mat: &mut Matrix<char>) {
    // transpose
    for i in 0..mat.rows() {
        for j in 0..mat.cols() {
            if j > i {
                break;
            }

            let tmp1 = mat.get(i, j).copied().unwrap();
            let tmp2 = mat.get(j, i).copied().unwrap();

            let el1 = mat.get_mut(i, j).unwrap();
            *el1 = tmp2;

            let el2 = mat.get_mut(j, i).unwrap();
            *el2 = tmp1;
        }
    }

    // reverse rows
    for i in 0..mat.rows() {
        let row = mat.get_row(i).unwrap().copied().collect_vec();
        for (j, char) in row.iter().rev().enumerate() {
            let el = mat.get_mut(i, j).unwrap();
            *el = *char;
        }
    }
}

fn task_2(input: &str) -> u32 {
    let mut mat: Matrix<char> = Matrix::from_iter(
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
        input.chars().filter(|c| ['.', 'O', '#'].contains(c)),
    );

    let mut seen_states: Vec<Matrix<char>> = vec![mat.clone()];

    loop {
        for _ in 0..4 {
            move_stones(&mut mat);
            rotate_mat(&mut mat);
        }
        if let Some(index) = seen_states.iter().position(|x| x == &mat) {
            let cycle_length = seen_states.len() - index;
            let cycle_start = index;
            let final_mat =
                seen_states[cycle_start + (1_000_000_000 - cycle_start) % cycle_length].clone();

            let mut res = 0;
            for i in (0..final_mat.rows()).rev() {
                let row = final_mat.get_row(final_mat.rows() - 1 - i).unwrap();
                res += (i + 1) * row.filter(|char| **char == 'O').count()
            } 
            return res as u32;
        }
        seen_states.push(mat.clone());
    }
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
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let result = task_1(input);
        assert_eq!(result, 136);
    }

    #[test]
    fn task_2_works() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let result = task_2(input);
        assert_eq!(result, 64);
    }
}
