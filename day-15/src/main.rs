use itertools::Itertools;

fn hash(str: &str) -> u32 {
    str.chars()
        .filter(|char| *char != '\n')
        .fold(0, |mut acc, char| {
            let ascii = char.to_ascii_lowercase() as u8;
            acc += ascii as u32;
            acc *= 17;
            acc = acc % 256;
            acc
        })
}

fn task_1(input: &str) -> u32 {
    input.split(',').map(|str| hash(str)).sum()
}

fn task_2(input: &str) -> u32 {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];
    for str in input.split(',') {
        if str.contains("=") {
            let (str, focal_len) = str.splitn(2, "=").collect_tuple().unwrap();

            let focal_len = focal_len.parse::<u32>().unwrap();
            let hash = hash(str);

            let box_vec = boxes.get_mut(hash as usize).unwrap();

            if let Some(i) = box_vec.iter().position(|(key, _)| **key == *str) {
                box_vec.remove(i);
                box_vec.insert(i, (str, focal_len));
            } else {
                box_vec.push((str, focal_len));
            }
        } else {
            let (str, _) = str.splitn(2, "-").collect_tuple().unwrap();
            let hash = hash(str);

            let box_vec = boxes.get_mut(hash as usize).unwrap();

            if let Some(i) = box_vec.iter().position(|(key, _)| **key == *str) {
                box_vec.remove(i);
            }
        }
    }

    boxes.iter().enumerate().fold(0, |mut acc, (i, vec)| {
        for (j, (_, focal_len)) in vec.iter().enumerate() {
            acc += ((i + 1) * (j + 1) * (*focal_len) as usize) as u32
        }
        acc
    })
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = task_1(input);
        assert_eq!(result, 1320);
    }

    #[test]
    fn task_2_works() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = task_2(input);
        assert_eq!(result, 145);
    }
}
