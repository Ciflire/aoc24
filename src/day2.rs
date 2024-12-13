use std::{fs, ops::Not};

pub fn day2_1() -> u64 {
    let mut report_safe = 0;

    let file = fs::read_to_string("src/data/day2.txt").unwrap_or_default();

    let reports = file
        .lines()
        .filter(|s| s.is_empty().not())
        .map(|s| {
            s.split(' ')
                .map(|e| e.parse().unwrap_or_default())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    reports.iter().for_each(|report| {
        if is_report_safe(report) {
            report_safe += 1
        }
    });
    report_safe
}

fn is_report_safe(report: &Vec<u64>) -> bool {
    let n = report.len();
    let mut list_order = std::cmp::Ordering::Equal;
    let mut report_safe = true;
    for i in 0..n - 1 {
        match report[i].partial_cmp(&report[i + 1]) {
            Some(order) => match order {
                std::cmp::Ordering::Less => {
                    if list_order.eq(&std::cmp::Ordering::Equal) {
                        list_order = std::cmp::Ordering::Less;
                    }
                    if order != list_order || report[i].abs_diff(report[i + 1]) > 3 {
                        report_safe = false;
                        break;
                    }
                }
                std::cmp::Ordering::Equal => {
                    report_safe = false;
                    break;
                }
                std::cmp::Ordering::Greater => {
                    if list_order.eq(&std::cmp::Ordering::Equal) {
                        list_order = std::cmp::Ordering::Greater;
                    }
                    if order != list_order || report[i].abs_diff(report[i + 1]) > 3 {
                        report_safe = false;
                        break;
                    }
                }
            },
            None => todo!(),
        }
    }
    report_safe
}

pub fn day2_2() -> u64 {
    let mut report_safe = 0;

    let file = fs::read_to_string("src/data/day2.txt").unwrap_or_default();

    let reports = file
        .lines()
        .filter(|s| s.is_empty().not())
        .map(|s| {
            s.split(' ')
                .map(|e| e.parse().unwrap_or_default())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    reports.iter().for_each(|report| {
        let n = report.len();
        if is_report_safe(report).not() {
            for i in 0..n {
                let mut modified_report = report.clone();
                modified_report.remove(i);
                if is_report_safe(&modified_report) {
                    report_safe += 1;
                    break;
                }
            }
        } else {
            report_safe += 1;
        }
    });

    report_safe
}

#[cfg(test)]
mod test {
    use crate::day2::{day2_1, day2_2};

    #[test]
    fn test_day_2_1() {
        assert_eq!(day2_1(), 224);
    }

    #[test]
    fn test_day_2_2() {
        assert_eq!(day2_2(), 293);
    }
}
