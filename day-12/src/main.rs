use std::iter;
use cached::proc_macro::cached;

use itertools::Itertools;

#[cached]
fn count(str: String, groups: Vec<u32>) -> u64 {
    if str == "" {
        if groups.is_empty() {
            // no springs left and expecting none
            return 1;
        } else {
            // no springs left but expecting more
            return 0;
        }
    }
    if groups.is_empty() {
        if str.contains('#') {
            // expecting no more groups but str still contains groups
            return 0;
        } else {
            // expecting no more groups and str doesnt contain any
            return 1;
        }
    }
    
    let mut result = 0;

    // case 1: operational spring
    if ['.', '?'].contains(&str.chars().next().unwrap()) {
        // skip operational spring and check for the rest of string
        result += count(str.get(1..).unwrap().to_string(), groups.clone());
    }

    // case 2: broken spring
    if ['#', '?'].contains(&str.chars().next().unwrap()) {
        // only valid if:
        // - there are enough springs left
        // - the required block size can be achived (no '.' in first n chars of str)
        // - spring after block must be '.' (either no springs left or next char is not '#')
        if groups[0] as usize <= str.len()
            && !str.get(..groups[0] as usize).unwrap().contains('.')
            && (groups[0] as usize == str.len()
                || str.chars().collect_vec()[groups[0] as usize] != '#')
        {
            // the block is valid -> remove n+1 chars from string (because after group there must be '.') and the first group size from groups
            if let Some(str) = str.get((groups[0] + 1) as usize..) {
                result += count(
                    str.to_string(),
                    groups[1..].to_vec(),
                )
            } else {
                result += count(
                    "".to_string(),
                    groups[1..].to_vec(),
                )
            }
        }
    }

    result
}

fn task_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, nums): (&str, &str) = line.split(" ").collect_tuple().unwrap();
            let group_sizes: Vec<u32> = nums.split(",").map(|n| n.parse().unwrap()).collect_vec();

            count(springs.to_string(), group_sizes)
        })
        .sum()
}

fn task_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, nums): (&str, &str) = line.split(" ").collect_tuple().unwrap();
            let group_sizes: Vec<u32> = nums.split(",").map(|n| n.parse().unwrap()).collect_vec();
            let springs = [springs; 5].join("?");
            let group_sizes = iter::repeat(group_sizes).take(5).flatten().collect_vec();

            count(springs, group_sizes) as u64
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
