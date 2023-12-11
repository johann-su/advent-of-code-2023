use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, space1, u64},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn can_be_won(&self, button_time: u64) -> bool {
        (self.time - button_time) * button_time > self.record
    }
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = preceded(
        permutation((alpha1, tag(":"), space1)),
        separated_list1(space1, u64),
    )(input)?;
    let (input, records) = preceded(
        permutation((multispace0, alpha1, tag(":"), space1)),
        separated_list1(space1, u64),
    )(input)?;

    let races = times
        .iter()
        .enumerate()
        .map(|(i, time)| {
            let record = records.get(i).unwrap();
            Race {
                time: *time,
                record: *record,
            }
        })
        .collect::<Vec<_>>();

    Ok((input, races))
}

fn task_1(input: &str) -> u64 {
    let races = parse_races(input).unwrap().1;

    races
        .iter()
        .map(|race| {
            (1..race.time).fold(
                0,
                |acc, time| {
                    if race.can_be_won(time) {
                        acc + 1
                    } else {
                        acc
                    }
                },
            )
        })
        .product()
}

fn parse_single_race(input: &str) -> IResult<&str, Race> {
    let (input, times) = preceded(
        permutation((alpha1, tag(":"), space1)),
        separated_list1(space1, digit1),
    )(input)?;
    let (input, records) = preceded(
        permutation((multispace0, alpha1, tag(":"), space1)),
        separated_list1(space1, digit1),
    )(input)?;

    let time = times
        .iter()
        .fold("".to_string(), |acc, i| format!["{acc}{i}"])
        .parse::<u64>()
        .unwrap();
    let record = records
        .iter()
        .fold("".to_string(), |acc, i| format!["{acc}{i}"])
        .parse::<u64>()
        .unwrap();

    Ok((input, Race { time, record }))
}

fn task_2(input: &str) -> u64 {
    let race = parse_single_race(input).unwrap().1;

    (1..race.time).fold(
        0,
        |acc, time| {
            if race.can_be_won(time) {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn main() {
    let input = include_str!["./data.txt"];

    println!("task 1: {}", task_1(input));
    println!("task 2: {}", task_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = task_1(input);
        assert_eq![result, 288];
    }

    #[test]
    fn task_2_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = task_2(input);
        assert_eq![result, 71503];
    }
}
