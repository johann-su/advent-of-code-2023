use std::vec;

use itertools::Itertools;

fn task_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (springs, nums): (&str, &str) = line.split(" ").collect_tuple().unwrap();
            let group_sizes: Vec<u32> = nums.split(",").map(|n| n.parse().unwrap()).collect_vec();

            let mut all_combinations: Vec<String> = vec![springs.to_owned()];

            while all_combinations.iter().any(|str| str.contains("?")) {
                let str = all_combinations.remove(0);
                if str.contains("?") {
                    all_combinations.push(str.clone().replacen("?", ".", 1));
                    all_combinations.push(str.clone().replacen("?", "#", 1));
                } else {
                    all_combinations.push(str);
                }
            }

            all_combinations
                .iter()
                .filter(|line| {
                    let mut num_groups: Vec<u32> = vec![];
                    for (key, group) in &line.chars().group_by(|char| *char == '#') {
                        if key == true {
                            num_groups.push(group.collect_vec().len() as u32)
                        }
                    }

                    num_groups == group_sizes
                })
                .collect_vec()
                .len() as u32
        })
        .sum()
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
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = task_1(input);
        assert_eq!(result, 21);
    }
}
