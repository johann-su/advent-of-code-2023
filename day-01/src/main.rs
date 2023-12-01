use std::collections::HashMap;

fn task_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let mut res: Vec<_> = l.match_indices(numbers).collect();

            res.sort_by(|a,b| a.0.cmp(&b.0));

            if res.len() > 0 {
                let first = res.first().unwrap().1.parse::<u32>().unwrap();
                let last = res.last().unwrap().1.parse::<u32>().unwrap();

                10 * first + last
            } else {
                0
            }
        })
        .sum::<u32>()
}

fn task_2(input: &str) -> u32 {
    let mut nums = HashMap::new();
    nums.insert("one", "one1one");
    nums.insert("two", "two2two");
    nums.insert("three", "three3three");
    nums.insert("four", "four4four");
    nums.insert("five", "five5five");
    nums.insert("six", "six6six");
    nums.insert("seven", "seven7seven");
    nums.insert("eight", "eight8eight");
    nums.insert("nine", "nine9nine");

    let inp: String = input
        .lines()
        .map(|l| {
            let mut matches: Vec<(usize, &str)> = Vec::new();

            for n in nums.iter() {
                l.match_indices(n.0)
                    .into_iter()
                    .for_each(|e| matches.push(e));
            }

            matches.sort_by(|a, b| a.0.cmp(&b.0));

            let mut res = l.to_string();
            if matches.len() > 0 {
                let first = matches.first().unwrap();
                let last = matches.last().unwrap();

                res = res.replacen(first.1, nums.get(first.1).unwrap(), 1);
                res = res.replace(last.1, nums.get(last.1).unwrap());
            }
            res
        })
        .reduce(|acc, s| acc + "\n" + &s)
        .unwrap();

    task_1(&inp)
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = super::task_1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn task_2_works() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = super::task_2(input);
        assert_eq!(result, 281);

        let input = "twone";

        let result = super::task_2(input);
        assert_eq!(result, 21);
    }
}
