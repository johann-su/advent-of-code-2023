use itertools::Itertools;

fn task_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (springs, nums): (&str, &str) = line.split(" ").collect_tuple().unwrap();
            let group_sizes: Vec<u32> = nums.split(",").map(|n| n.parse().unwrap()).collect_vec();

            let mut combinations: Vec<String> = vec![springs.to_string()];

            while let Some((i, line)) = combinations.iter().find_position(|line| line.contains('?'))
            {
                let line = line.clone();
                combinations.remove(i);
                combinations.push(line.replacen("?", "#", 1));
                combinations.push(line.replacen("?", ".", 1));
            }

            combinations.iter().fold(0, |acc, line| {
                if line
                    .chars()
                    .group_by(|char| *char == '#')
                    .into_iter()
                    .filter_map(|(key, group)| {
                        if key == true {
                            Some(group.count() as u32)
                        } else {
                            None
                        }
                    })
                    .collect_vec()
                    == group_sizes
                {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .sum::<u32>()
}

fn task_2(input: &str) -> u32 {
    0
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
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = task_1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn task_2_works() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = task_2(input);
        assert_eq!(result, 525152);
    }
}
