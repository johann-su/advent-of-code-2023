use itertools::Itertools;
use nom::{
    character::complete::{i32, multispace1, space1},
    multi::separated_list1,
    IResult,
};

fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, sequences) = separated_list1(multispace1, separated_list1(space1, i32))(input)?;

    Ok((input, sequences))
}

fn task_1(input: &str) -> i32 {
    let sequences = parse_sequences(input).unwrap().1;

    sequences
        .iter()
        .map(|sequence| {
            let mut i = 0;
            let mut diffs: Vec<Vec<i32>> = vec![sequence.clone()];

            while !diffs[i].iter().all(|x| *x == 0) {
                let prev_seq = diffs.get(i).unwrap();
                let mut next_seq: Vec<i32> = vec![];
                for (i, j) in prev_seq.iter().tuple_windows() {
                    next_seq.push(j - i);
                }
                diffs.push(next_seq);
                i += 1;
            }

            diffs.last_mut().unwrap().push(0);

            for i in (0..diffs.len() - 1).rev() {
                let prev_range = diffs.get(i + 1).unwrap();
                let curr_range = diffs.get(i).unwrap();

                let new_val: i32 = prev_range.last().unwrap() + curr_range.last().unwrap();

                let curr_range = diffs.get_mut(i).unwrap();
                curr_range.push(new_val);
            }

            diffs[0].last().unwrap().clone()
        })
        .sum()
}

fn task_2(input: &str) -> i32 {
    let sequences = parse_sequences(input).unwrap().1;

    sequences
        .iter()
        .map(|sequence| {
            let mut i = 0;
            let mut diffs: Vec<Vec<i32>> = vec![sequence.clone()];

            while !diffs[i].iter().all(|x| *x == 0) {
                let prev_seq = diffs.get(i).unwrap();
                let mut next_seq: Vec<i32> = vec![];
                for (i, j) in prev_seq.iter().tuple_windows() {
                    next_seq.push(j - i);
                }
                diffs.push(next_seq);
                i += 1;
            }

            diffs.last_mut().unwrap().push(0);

            for i in (0..diffs.len() - 1).rev() {
                let prev_range = diffs.get(i + 1).unwrap();
                let curr_range = diffs.get(i).unwrap();

                let new_val: i32 = curr_range.first().unwrap() - prev_range.first().unwrap();

                let curr_range = diffs.get_mut(i).unwrap();
                curr_range.insert(0, new_val);
            }

            diffs[0].first().unwrap().clone()
        })
        .sum()
}

fn main() {
    let input = include_str!["./data.txt"];

    println!("task 1: {}", task_1(input));
    println!("task 2: {}", task_2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_1_works() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = task_1(input);
        assert_eq![result, 114];
    }

    #[test]
    fn task_2_works() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = task_2(input);
        assert_eq![result, 2];
    }
}
