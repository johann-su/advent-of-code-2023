use std::{fmt::Display, collections::HashMap, char};

use simple_matrix::Matrix;

fn print_mat<T: Display>(mat: &Matrix<T>) {
    println!("{}x{}", mat.rows(), mat.cols());
    for i in 0..mat.rows() {
        for j in 0..mat.cols() {
            print!("{} ", mat.get(i, j).unwrap());
        }
        println!("");
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Horizontal,
    Vertical
}

fn neighborhood(mat: &Matrix<char>, p: (usize, usize)) -> Vec<Option<char>> {
    let mut n: Vec<Option<char>> = vec![];
    if p.0 > 0 {
        n.push(mat.get(p.0-1, p.1).copied())
    } else {
        n.push(None);
    }
    n.push(mat.get(p.0, p.1+1).copied());
    n.push(mat.get(p.0+1, p.1).copied());
    if p.1 > 0 {
        n.push(mat.get(p.0, p.1-1).copied())
    } else {
        n.push(None);
    }

    n
}

fn activate(mat: &mut Matrix<char>, p: (usize, usize), d: Dir) {
    if let Some(char) = mat.get_mut(p.0, p.1) {
        match char {
            '/' => {
                let n = neighborhood(mat, p);
                if let Some(t) = n.get(0).unwrap() {
                    
                }
                if let Some(l) = n.get(3).unwrap() {
                    
                }
            },
            '\\' => {
                if let Some('#') = mat.get(p.0-1, p.1) {
                    activate(mat, (p.0, p.1+1), Dir::Horizontal)
                } 
                if let Some('#') = mat.get(p.0, p.1+1) {
                    activate(mat, (p.0-1, p.1), Dir::Vertical)
                } 
                if let Some('#') = mat.get(p.0+1, p.1) {
                    activate(mat, (p.0, p.1-1), Dir::Horizontal)
                }
                if let Some('#') = mat.get(p.0, p.1-1) {
                    activate(mat, (p.0+1, p.1), Dir::Vertical)
                }
            }
            '|' => {
                if let Some('#') = mat.get(p.0-1, p.1) {
                    activate(mat, (p.0+1, p.1), d)
                } 
                if let Some('#') = mat.get(p.0, p.1+1) {
                    activate(mat, (p.0-1, p.1), Dir::Vertical);
                    activate(mat, (p.0+1, p.1), Dir::Vertical);
                } 
                if let Some('#') = mat.get(p.0+1, p.1) {
                    activate(mat, (p.0-1, p.1), d)
                }
                if let Some('#') = mat.get(p.0, p.1-1) {
                    activate(mat, (p.0-1, p.1), Dir::Vertical);
                    activate(mat, (p.0+1, p.1), Dir::Vertical);
                }
            },
            '-' => {
               if let Some('#') = mat.get(p.0-1, p.1) {
                    activate(mat, (p.0, p.1-1), Dir::Horizontal);
                    activate(mat, (p.0, p.1+1), Dir::Horizontal);
                } 
                if let Some('#') = mat.get(p.0, p.1+1) {
                    activate(mat, (p.0, p.1-1), d);
                } 
                if let Some('#') = mat.get(p.0+1, p.1) {
                    activate(mat, (p.0, p.1-1), Dir::Horizontal);
                    activate(mat, (p.0, p.1+1), Dir::Horizontal);
                }
                if let Some('#') = mat.get(p.0, p.1-1) {
                    activate(mat, (p.0, p.1+1), d);
                } 
            },
            '.' | '#' => {
                if *char == '.' {
                    *char = '#';
                }
                match d {
                    Dir::Horizontal => {
                        activate(mat, (p.0-1,p.1), d);
                        activate(mat, (p.0+1,p.1), d);
                    },
                    Dir::Vertical => {
                        activate(mat, (p.0,p.1-1), d);
                        activate(mat, (p.0,p.1+1), d);
                    }
                }
            },
            _ => panic!("invalid char")
        }
    }
}

fn task_1(input: &str) -> u32 {
    let mut mat: Matrix<char> = Matrix::from_iter(
        input.lines().count(),
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .filter(|char| *char != '\n')
            .count(),
        input.chars().filter(|char| *char != '\n'),
    );
    activate(&mut mat, (0,0), Dir::Horizontal);

    print_mat(&mat);

    todo!()
}

fn main() {
    let input = include_str!("./data.txt");

    println!("task 1: {}", task_1(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_1_works() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        let result = task_1(input);
        assert_eq!(result, 46);
    }
}
