use std::vec;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, one_of},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

type Vec2 = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TileType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    t: TileType,
    position: Vec2,
}

impl Tile {
    fn is_above(&self, other: Tile) -> bool {
        self.position.1 > other.position.1 && self.position.0 == other.position.0
    }

    fn process_north(&self, other: Tile) -> bool {
        // South
        if self.is_above(other) {
            match other.t {
                TileType::SouthEast => true,
                TileType::SouthWest => true,
                TileType::NorthSouth => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_below(&self, other: Tile) -> bool {
        self.position.1 < other.position.1 && self.position.0 == other.position.0
    }

    fn process_south(&self, other: Tile) -> bool {
        // North
        if self.is_below(other) {
            match other.t {
                TileType::NorthEast => true,
                TileType::NorthWest => true,
                TileType::NorthSouth => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_right(&self, other: Tile) -> bool {
        self.position.0 > other.position.0 && self.position.1 == other.position.1
    }

    fn process_west(&self, other: Tile) -> bool {
        // East
        if self.is_right(other) {
            match other.t {
                TileType::EastWest => true,
                TileType::NorthEast => true,
                TileType::SouthEast => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_left(&self, other: Tile) -> bool {
        self.position.0 < other.position.0 && self.position.1 == other.position.1
    }

    fn process_east(&self, other: Tile) -> bool {
        // West
        if self.is_left(other) {
            match other.t {
                TileType::EastWest => true,
                TileType::NorthWest => true,
                TileType::SouthWest => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn can_be_connected(&self, other: Tile) -> bool {
        match self.t {
            TileType::NorthSouth => self.process_north(other) || self.process_south(other),
            TileType::EastWest => self.process_east(other) || self.process_west(other),
            TileType::NorthEast => self.process_north(other) || self.process_east(other),
            TileType::NorthWest => self.process_north(other) || self.process_west(other),
            TileType::SouthWest => self.process_south(other) || self.process_west(other),
            TileType::SouthEast => self.process_south(other) || self.process_east(other),
            TileType::Start => {
                if self.is_above(other)
                    || self.is_below(other)
                    || self.is_left(other)
                    || self.is_right(other)
                {
                    match other.t {
                        TileType::Ground => false,
                        _ => true,
                    }
                } else {
                    false
                }
            }
            TileType::Ground => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Maze(Vec<Tile>);

impl Maze {
    fn find_start(&self) -> Option<Tile> {
        self.0
            .iter()
            .find(|tile| tile.t == TileType::Start)
            .copied()
    }

    fn get_by_position(&self, position: Vec2) -> Option<Tile> {
        self.0
            .iter()
            .find(|tile| tile.position == (position.0, position.1))
            .copied()
    }

    fn get_row(&self, position: Vec2) -> Option<Vec<Tile>> {
        let row = self
            .0
            .iter()
            .filter(|tile| tile.position.1 == position.1)
            .copied()
            .collect_vec();
        if !row.is_empty() {
            Some(row)
        } else {
            None
        }
    }

    fn get_neighborhood(&self, point: Tile) -> Vec<Tile> {
        let mut neighborhood: Vec<Tile> = vec![];
        if let Some(n) = self.get_by_position((point.position.0 - 1, point.position.1 - 1)) {
            // Top left
            neighborhood.push(n);
        }
        if let Some(n) = self.get_by_position((point.position.0, point.position.1 - 1)) {
            // Top center
            neighborhood.push(n);
        }
        if let Some(n) = self.get_by_position((point.position.0 + 1, point.position.1 - 1)) {
            // Top right
            neighborhood.push(n);
        }
        if let Some(n) = self.get_by_position((point.position.0 - 1, point.position.1)) {
            // Middle left
            neighborhood.push(n);
        }
        if let Some(n) = self.get_by_position((point.position.0 + 1, point.position.1)) {
            // Middle right
            neighborhood.push(n);
        }
        if let Some(n) = self.get_by_position((point.position.0 - 1, point.position.1 + 1)) {
            // Bottom left
            neighborhood.push(n);
        }
        if let Some(n) = self.get_by_position((point.position.0, point.position.1 + 1)) {
            // Bottom center
            neighborhood.push(n);
        }
        if let Some(n) = self.get_by_position((point.position.0 + 1, point.position.1 + 1)) {
            // Bottom right
            neighborhood.push(n);
        }

        neighborhood
    }

    fn bfs(&self, start: Tile) -> Vec<Tile> {
        let mut visited_nodes: Vec<Tile> = vec![];
        let mut queue: Vec<Tile> = vec![start];

        while !queue.is_empty() {
            let curr_node = queue.remove(0);
            if visited_nodes.contains(&curr_node) {
                continue;
            }
            let mut connections: Vec<Tile> = vec![];

            for n in self.get_neighborhood(curr_node) {
                if curr_node.can_be_connected(n) {
                    connections.push(n);
                }
            }

            visited_nodes.push(curr_node);
            queue.append(&mut connections);
        }

        visited_nodes
    }

    fn intersections(&self, point: Tile, path: &Vec<Tile>) -> u32 {
        let row = self.get_row(point.position).unwrap();
        let mut intersects = 0;
        let mut angle_intersects: Vec<Tile> = vec![];

        for tile in row
            .iter()
            .filter(|tile| tile.is_left(point) && path.contains(tile))
        {
            match tile.t {
                TileType::NorthSouth => {intersects+=1},
                TileType::NorthEast => {
                    angle_intersects.push(*tile);
                },
                TileType::SouthEast => {
                    angle_intersects.push(*tile);
                },
                TileType::NorthWest => {
                    if let Some(t) = angle_intersects.last() {
                        if t.t == TileType::SouthEast {
                            intersects+=1;
                        }
                    }
                },
                TileType::SouthWest => {
                    if let Some(t) = angle_intersects.last() {
                        if t.t == TileType::NorthEast {
                            intersects+=1;
                        }
                    }
                },
                _ => {}
            }
        }

        intersects
    }

    fn is_enclosed(&self, point: Tile, path: &Vec<Tile>) -> bool {
        if path.contains(&point) {
            return false;
        }

        self.intersections(point, path) % 2 == 1
    }
}

fn parse_tile(input: &str, position: Vec2) -> IResult<&str, Tile> {
    map_res(anychar, |char| match char {
        '|' => Ok(Tile {
            t: TileType::NorthSouth,
            position,
        }),
        '-' => Ok(Tile {
            t: TileType::EastWest,
            position,
        }),
        'L' => Ok(Tile {
            t: TileType::NorthEast,
            position,
        }),
        'J' => Ok(Tile {
            t: TileType::NorthWest,
            position,
        }),
        '7' => Ok(Tile {
            t: TileType::SouthWest,
            position,
        }),
        'F' => Ok(Tile {
            t: TileType::SouthEast,
            position,
        }),
        '.' => Ok(Tile {
            t: TileType::Ground,
            position,
        }),
        'S' => Ok(Tile {
            t: TileType::Start,
            position,
        }),
        _ => Err("not a valid char"),
    })(input)
}

fn parse_maze(input: &str) -> IResult<&str, Maze> {
    let c = ['|', '-', 'L', 'J', '7', 'F', '.', 'S'];
    let (input, chars) = separated_list1(tag("\n"), many1(one_of(&c[..])))(input)?;
    let mut tiles: Vec<Tile> = vec![];

    for (x, col) in chars.iter().enumerate() {
        for (y, char) in col.iter().filter(|c| **c != '\n').enumerate() {
            tiles.push(
                parse_tile(&char.to_string(), (y as i32, x as i32))
                    .unwrap()
                    .1,
            );
        }
    }

    Ok((input, Maze(tiles)))
}

fn task_1(input: &str) -> u32 {
    let maze = parse_maze(input).unwrap().1;
    let start = maze.find_start().unwrap();
    let path = maze.bfs(start);

    (path.len() / 2) as u32
}

fn task_2(input: &str) -> u32 {
    let maze = parse_maze(input).unwrap().1;
    let start = maze.find_start().unwrap();
    let path = maze.bfs(start);

    maze.0
        .iter()
        .filter(|tile| maze.is_enclosed(**tile, &path))
        .fold(0, |acc, _| acc + 1)
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
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        let result = task_1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn task_2_works() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let result = task_2(input);
        assert_eq!(result, 4);

        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        let result = task_2(input);
        assert_eq!(result, 4);

        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let result = task_2(input);
        assert_eq!(result, 10);
    }
}
