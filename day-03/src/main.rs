use regex::Regex;

fn task_1(input: &str) -> u32 {
    let mat: Vec<Vec<String>> = input.lines().map(|l| {
        let mut chars: Vec<String> = Vec::new();

        for c in l.chars() {
            chars.push(c.to_string());
        }

        let re = Regex::new(r"(\d+)").unwrap();
        for m in re.find_iter(l) {
            for i in m.start()..m.end() {
                chars[i] = m.as_str().to_string();
            }
        }

        chars
    })
    .collect();

    let mut total = 0;
    for (x, col) in mat.iter().enumerate() {
        for (y, c) in col.iter().enumerate() {
            if c.parse::<u32>().is_ok() || *c == "." {continue};

            let mut neighbors: Vec<u32> = Vec::new();
            let mut last_elem = 0;
            for column in mat.iter().skip(x-1).take(3) {
                for elem in column.iter().skip(y-1).take(3) {
                    let parsed_elem = elem.parse::<u32>();
                    if parsed_elem.is_ok() {
                        let num = parsed_elem.unwrap();
                        if num != last_elem {
                            neighbors.push(num);
                            last_elem = num;
                        }
                    };
                }
            }
            
            total += neighbors.iter().sum::<u32>();
        }
    }

    total
}

fn task_2(input: &str) -> u32 {
    let mat: Vec<Vec<String>> = input.lines().map(|l| {
        let mut chars: Vec<String> = Vec::new();

        for c in l.chars() {
            chars.push(c.to_string());
        }

        let re = Regex::new(r"(\d+)").unwrap();
        for m in re.find_iter(l) {
            for i in m.start()..m.end() {
                chars[i] = m.as_str().to_string();
            }
        }

        chars
    })
    .collect();

    let mut total = 0;
    for (x, col) in mat.iter().enumerate() {
        for (y, c) in col.iter().enumerate() {
            if *c != "*" {continue};

            let mut neighbors: Vec<u32> = Vec::new();
            let mut last_elem = 0;
            for column in mat.iter().skip(x-1).take(3) {
                for elem in column.iter().skip(y-1).take(3) {
                    let parsed_elem = elem.parse::<u32>();
                    if parsed_elem.is_ok() {
                        let num = parsed_elem.unwrap();
                        if num != last_elem {
                            neighbors.push(num);
                            last_elem = num;
                        }
                    };
                }
            }
            
            if neighbors.len() > 1 {
                total += neighbors.iter().product::<u32>();
            }
        }
    }

    total
}

fn main() {
    let input = include_str!("./data.txt");

    println!("task 1: {}", task_1(input));
    println!("task 2: {}", task_2(input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn task_1_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = super::task_1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn task_2_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = super::task_2(input);
        assert_eq!(result, 467835);
    }
}