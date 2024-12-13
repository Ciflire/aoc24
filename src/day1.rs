use std::fs;
use std::ops::Not;

pub fn day1_1() -> u64 {
    let file: String = fs::read_to_string("src/data/day1.txt").unwrap_or_default();

    let mut res = 0;

    let mut left_column: Vec<u64> = Vec::new();
    let mut right_column: Vec<u64> = Vec::new();

    let lines = file
        .split('\n')
        .filter(|line| line.is_empty().not())
        .map(|line| {
            line.split("   ")
                .map(|i| i.parse::<u64>().unwrap_or_default())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    lines.iter().for_each(|line| {
        left_column.push(line[0]);
        right_column.push(line[1]);
    });

    left_column.sort();
    right_column.sort();

    left_column
        .iter()
        .zip(right_column.iter())
        .for_each(|(l, r)| {
            res += l.abs_diff(*r);
        });

    res
}

pub fn day1_2() -> u64 {
    let file: String = fs::read_to_string("src/data/day1.txt").unwrap_or_default();

    let mut res = 0;

    let mut left_column: Vec<u64> = Vec::new();
    let mut right_column: Vec<u64> = Vec::new();

    let lines = file
        .split('\n')
        .filter(|line| line.is_empty().not())
        .map(|line| {
            line.split("   ")
                .map(|i| i.parse::<u64>().unwrap_or_default())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    lines.iter().for_each(|line| {
        left_column.push(line[0]);
        right_column.push(line[1]);
    });

    left_column.sort();
    right_column.sort();

    left_column.iter().for_each(|e| {
        let occurences = right_column.iter().filter(|f| *f == e).count() as u64;

        res += occurences * e;
    });

    res
}

#[cfg(test)]
mod test {
    use crate::day1::{day1_1, day1_2};

    #[test]
    fn test_day_1_1() {
        assert_eq!(day1_1(), 2367773);
    }

    #[test]
    fn test_day_1_2() {
        assert_eq!(day1_2(), 21271939);
    }
}
