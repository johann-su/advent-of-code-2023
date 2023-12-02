use regex::Regex;

fn task_1(input: &str) -> u32 {
    let max_cubes = (12, 13, 14); // (R,G,B)

    input
        .lines()
        .map(|l| {
            let mut str = l.to_string();
            let re = Regex::new(r"^Game\s(\d*)").unwrap();
            let game_id = re
                .captures(l)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();

            str = str.split(": ").last().unwrap().to_string();

            let is_possible: bool = str
                .split(";")
                .map(|g| {
                    let re = Regex::new(r"([0-9]+\s[a-z]+)").unwrap();

                    let mut res = (0, 0, 0);
                    for capture in re.captures_iter(g) {
                        if let Some(m) = capture.get(0) {
                            let str_split: Vec<_> = m.as_str().split(" ").collect();

                            match str_split.get(1).unwrap() {
                                &"red" => res.0 = str_split.get(0).unwrap().parse().unwrap(),
                                &"green" => res.1 = str_split.get(0).unwrap().parse().unwrap(),
                                &"blue" => res.2 = str_split.get(0).unwrap().parse().unwrap(),
                                _ => {}
                            }
                        }
                    }

                    if res.0 <= max_cubes.0 && res.1 <= max_cubes.1 && res.2 <= max_cubes.2 {
                        true
                    } else {
                        false
                    }
                })
                .all(|e| e == true);

            if is_possible {
                game_id
            } else {
                0
            }
        })
        .sum::<u32>()
}

fn task_2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut str = l.to_string();
            str = str.split(": ").last().unwrap().to_string();

            let mut min_cubes = (0, 0, 0);
            for g in str.split(";") {
                let re = Regex::new(r"([0-9]+\s[a-z]+)").unwrap();

                for capture in re.captures_iter(g) {
                    if let Some(m) = capture.get(0) {
                        let str_split: Vec<_> = m.as_str().split(" ").collect();

                        match str_split.get(1).unwrap() {
                            &"red" => {
                                let red = str_split.get(0).unwrap().parse().unwrap();
                                if red > min_cubes.0 {
                                    min_cubes.0 = red
                                }
                            }
                            &"green" => {
                                let green = str_split.get(0).unwrap().parse().unwrap();
                                if green > min_cubes.1 {
                                    min_cubes.1 = green;
                                }
                            }
                            &"blue" => {
                                let blue = str_split.get(0).unwrap().parse().unwrap();
                                if blue > min_cubes.2 {
                                    min_cubes.2 = blue;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            min_cubes
        })
        .fold(0, |mut acc, e| {acc += e.0 * e.1 * e.2; acc})
}

fn main() {
    let input = include_str!("./data.txt");

    println!("task 1: {}", task_1(input));
    println!("task 2: {}", task_2(input));
}

#[cfg(test)]
mod tests {
    use crate::task_2;

    #[test]
    fn task_1_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = super::task_1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn task_2_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = task_2(input);
        assert_eq!(result, 2286);
    }
}
